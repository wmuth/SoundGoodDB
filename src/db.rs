use std::env;
use std::fmt;

use dotenvy::dotenv;
use sqlx::{
    postgres::PgPoolOptions,
    types::{time::OffsetDateTime, BigDecimal},
    PgPool, Postgres, Transaction,
};

const MAX_RENTALS_KEY: &str = "rent_max_count";
const POOL_CONNECTIONS: u32 = 5;

#[allow(dead_code)]
struct InstrumentType {
    instrument_type_id: i32,
    instrument_type: String,
}

/// `Instrument` matches the columns found in the database facilitating the use of [`sqlx::query_as!`]
#[allow(dead_code)]
pub struct Instrument {
    /// PK of instrument table
    instrument_id: i32,
    /// The type of the instrument, resolved to string through other table lookup
    instrument_type_id: i32,
    /// The brand which made the instrument e.g. "Steinway"
    brand: String,
    /// The model the instrument is e.g. "Alpha 160"
    model: String,
    /// The price to rent
    price: BigDecimal,
    /// The total count of how many the school has (including currently rented out)
    count: i32,
}

/// `Renting` matches the columns found in the database facilitating the use of [`sqlx::query_as!`]
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct Renting {
    /// PK of rent table
    rent_id: i32,
    /// The student who is renting
    student_id: i32,
    /// The instrument the student is renting
    instrument_id: i32,
    /// The date at which the renting started, with timezone
    start_date: OffsetDateTime,
    /// The date at which the renting ended, with timezone. Potentially NULL therefore option
    end_date: Option<OffsetDateTime>,
}

impl Instrument {
    /// Takes in the number which are available to rent and returns object data as String
    ///
    /// # Parameters
    /// - `available` The number of instruments which are avialble, e.g. self.count - rented
    ///
    /// # Returns
    /// The data of the object as well as the number available in a formatted string ready to be
    /// printed to the user.
    pub fn to_string(&self, available: i64) -> String {
        format!(
            "ID:{} => {} by {}. Price {:.2} with {} left to rent out of a total {}.",
            self.instrument_id, self.model, self.brand, self.price, available, self.count
        )
    }

    pub const fn get_id(&self) -> i32 {
        self.instrument_id
    }

    pub const fn get_count(&self) -> i32 {
        self.count
    }
}

impl Renting {
    pub const fn get_id(&self) -> i32 {
        self.rent_id
    }
}

impl fmt::Display for Renting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Renting {} for student {} of instrument {} started at {}",
            self.rent_id, self.student_id, self.instrument_id, self.start_date
        )
    }
}

/// Sets up the connection to the database
///
/// # Parameters
/// - `DATABASE_URL` in a `.env` file located at the root of the project, see README
///
/// # Returns
/// - [`PgPool`] if setting up the connection and pool was successful
/// - [`sqlx::Error`] if there was an error
///
/// # Panics
/// The .env file is not found or the `DATABASE_URL` can not be read in that file the process will
/// panic as there is no way the program can continue with a failed database connection
pub async fn setup_conn() -> Result<PgPool, sqlx::Error> {
    dotenv().expect(".env file not found!");

    let db_url = env::var("DATABASE_URL").expect("DATABSE_URL not set in .env!");

    let pool = PgPoolOptions::new()
        .max_connections(POOL_CONNECTIONS)
        .connect(&db_url)
        .await?;

    Ok(pool)
}

/// Lists all instruments in the database
///
/// # Parameters
/// - `tx` the [`Transaction`] to execute queries with
///
/// # Returns
/// - [`Vec<Instrument>`] if rows are found
/// - [`sqlx::Error`] if there is an sql error
pub async fn list_all(tx: &mut Transaction<'_, Postgres>) -> Result<Vec<Instrument>, sqlx::Error> {
    sqlx::query_as!(Instrument, "SELECT * FROM instruments")
        .fetch_all(&mut **tx)
        .await
}

/// Lists all instruments of a certain type
///
/// # Parameters
/// - `tx` the [`Transaction`] to execute queries with
/// - `t` the type of instrument to list as pattern, e.g. 'guitar' or 'gui%'
///
/// # Returns
/// - [`Vec<Instrument>`] if rows are found
/// - [`sqlx::Error`] if there is an sql error
pub async fn list_type(
    tx: &mut Transaction<'_, Postgres>,
    t: String,
) -> Result<Vec<Instrument>, sqlx::Error> {
    let r = sqlx::query_as!(
        InstrumentType,
        "SELECT * FROM instrument_types WHERE instrument_type LIKE $1;",
        t
    )
    .fetch_one(&mut **tx)
    .await?;

    sqlx::query_as!(
        Instrument,
        "SELECT * FROM instruments where instrument_type_id = $1;",
        r.instrument_type_id
    )
    .fetch_all(&mut **tx)
    .await
}

/// Counts the number of rentals of a certain instrument id
///
/// # Parameters
/// - `tx` the [`Transaction`] to execute queries with
/// - `i_id` the id of the instrument to count
///
/// # Returns
/// - [`i64`] the number of rentals which was found
/// - [`sqlx::Error`] if there is an sql error
pub async fn count_instrument_rentals(
    tx: &mut Transaction<'_, Postgres>,
    i_id: i32,
) -> Result<i64, sqlx::Error> {
    let r = sqlx::query!(
        "SELECT COUNT(*) AS count FROM rentings WHERE instrument_id = $1 AND end_date IS NULL;",
        i_id
    )
    .fetch_one(&mut **tx)
    .await?
    .count
    .ok_or(sqlx::Error::ColumnNotFound(String::from("count")))?;

    Ok(r)
}

/// Counts the number of rentals of a certain user id
///
/// # Parameters
/// - `tx` the [`Transaction`] to execute queries with
/// - `u_id` the id of the user to count
///
/// # Returns
/// - [`i64`] the number of rentals which was found
/// - [`sqlx::Error`] if there is an sql error
pub async fn count_user_rentals(
    tx: &mut Transaction<'_, Postgres>,
    u_id: i32,
) -> Result<i64, sqlx::Error> {
    let r = sqlx::query!(
        "SELECT COUNT(*) AS count FROM rentings WHERE student_id = $1 AND end_date IS NULL;",
        u_id
    )
    .fetch_one(&mut **tx)
    .await?
    .count
    .ok_or(sqlx::Error::ColumnNotFound(String::from("count")))?;

    Ok(r)
}

/// Locks the rentings table wher user = u OR instrument = i
///
/// If the lock interferes with another transaction's lock this function will wait until the
/// currently ongoing transaction finishes before aquiring this lock.
///
/// # Parameters
/// - `tx` the [`Transaction`] to execute queries with
/// - `u` the id of the user to lock
/// - `i` the id of the instrument to lock
///
/// # Returns
/// - `()` if the lock was successful
/// - [`sqlx::Error`] if there is an sql error
pub async fn lock_rentings(
    tx: &mut Transaction<'_, Postgres>,
    u: i32,
    i: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "SELECT * FROM rentings WHERE student_id = $1 OR instrument_id = $2 FOR UPDATE;",
        u,
        i
    )
    .fetch_all(&mut **tx)
    .await?;

    Ok(())
}

/// Rents an instruments
///
/// Insers a new row into the rentings table to signal that a new renting has started
///
/// # Parameters
/// - `tx` the [`Transaction`] to execute queries with
/// - `u` the id of the user who is renting
/// - `i` the id of the instrument to rent
///
/// # Returns
/// - [`u64`] the number of rows affected (should always be 1)
/// - [`sqlx::Error`] if there is an sql error
pub async fn rent(tx: &mut Transaction<'_, Postgres>, u: i32, i: i32) -> Result<u64, sqlx::Error> {
    let r = sqlx::query!(
        "INSERT INTO rentings (student_id, instrument_id, start_date) VALUES ($1, $2, CURRENT_TIMESTAMP);",
        u,
        i
    )
    .execute(&mut **tx)
    .await?
    .rows_affected();

    Ok(r)
}

/// Finds rentings to terminate
///
/// Finds all rows which fullfill `student_id` = `u` AND `instrument_id` = `i`
///
/// # Parameters
/// - `tx` the [`Transaction`] to execute queries with
/// - `u` the id of the user who is renting
/// - `i` the id of the instrument to rent
///
/// # Returns
/// - [`Vec<Renting>`] the rows which were found
/// - [`sqlx::Error`] if there is an sql error
pub async fn find_to_terminate(
    tx: &mut Transaction<'_, Postgres>,
    u: i32,
    i: i32,
) -> Result<Vec<Renting>, sqlx::Error> {
    let r = sqlx::query_as!(
        Renting,
        "SELECT * FROM rentings WHERE student_id = $1 AND instrument_id = $2 AND end_date IS NULL;",
        u,
        i
    )
    .fetch_all(&mut **tx)
    .await?;

    Ok(r)
}

/// Terminates a renting based on the renting ID
///
/// Used by first finding rentings then terminating a specific one using its id
///
/// # Parameters
/// - `tx` the [`Transaction`] to execute queries with
/// - `id` the `rent_id` of the renting to terminate
///
/// # Returns
/// - [`u64`] the number of rows affected (should always be 1)
/// - [`sqlx::Error`] if there is an sql error
pub async fn terminate_rid(
    tx: &mut Transaction<'_, Postgres>,
    id: i32,
) -> Result<u64, sqlx::Error> {
    let r = sqlx::query!(
        "UPDATE rentings SET end_date = CURRENT_TIMESTAMP WHERE rent_id = $1;",
        id
    )
    .execute(&mut **tx)
    .await?
    .rows_affected();

    Ok(r)
}

/// Looks up the max allowed number of rentals from the database
///
/// # Parameters
/// - `tx` the [`Transaction`] to execute queries with
/// - [`MAX_RENTALS_KEY`] set in the file acts as the key to use in the table to find the value
///
/// # Returns
/// - [`String`] the string version of the value which can then be parsed to a numeric
/// - [`sqlx::Error`] tif there is an sql error
pub async fn get_max_rentals(tx: &mut Transaction<'_, Postgres>) -> Result<String, sqlx::Error> {
    let r = sqlx::query!(
        "SELECT value FROM business_rules WHERE name = $1;",
        MAX_RENTALS_KEY
    )
    .fetch_one(&mut **tx)
    .await?
    .value;

    Ok(r)
}
