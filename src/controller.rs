use sqlx::{PgPool, Postgres, Transaction};
use std::{fmt, num::ParseIntError};

use crate::{
    db::{self, Renting},
    repl::{self},
};

/// Controller struct which holds a DB connection pool and can execute command and run a repl
pub struct Controller<'a> {
    /// The pool of connections to use
    pool: PgPool,
    /// The transaction to execute with, created from pool
    transaction: Option<Transaction<'a, Postgres>>,
}

/// The commands available to be executed by the controller
///
/// Used by running [`Controller`]`.execute()` and passing the command
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    /// Begin new transaction
    Begin,
    /// Commit current transaction
    Commit,
    /// List optinally a specific type
    List(Option<String>),
    /// Rent for a user an instrument
    Rent(String, String),
    /// Roll back current transaction
    Rollback,
    /// Terminate a specific rent_id
    Terminate(String),
    /// Try to terminate a rent by user and instrument ids
    TryTerminate(String, String),
}

/// The results returned by [`Controller`]`.execute()`
///
/// Each variant corresponds to which type of command was executed and if data is also returned
/// then the data is contained within the variant type
///
/// For information on each variant see [`Command`]
#[derive(Debug, PartialEq, Eq)]
pub enum ControlResult {
    Begin,
    Commit,
    List(Vec<String>),
    Rent(u64),
    Rollback,
    Terminate(u64),
    TryTerminate(u64),
}

/// The errors returned by [`Controller`]`.execute()`
#[derive(Debug, PartialEq, Eq)]
pub enum ControlError {
    /// If another kind of error, e.g. [`sqlx::Error`] was returned then this contains the
    /// strinigified version of that error
    Converted(String),
    /// There are multiple rentings which could be terminated based on user and instrument
    TerminateMultiple(Vec<Renting>),
    /// The user has too many rentals to create a new one
    TooManyRentals,
    /// The transaction was none when DB function called
    TransactionNone,
}

impl fmt::Display for ControlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Converted(s) => write!(f, "{s}"),
            Self::TerminateMultiple(_) => write!(f, "Multiple rentings to terminate!"),
            Self::TooManyRentals => write!(f, "This user has too many rentals!"),
            Self::TransactionNone => write!(f, "Error! Transaction was None!"),
        }
    }
}

impl From<ParseIntError> for ControlError {
    fn from(value: ParseIntError) -> Self {
        Self::Converted(format!("ParseInt error: {value}"))
    }
}

impl From<sqlx::Error> for ControlError {
    fn from(value: sqlx::Error) -> Self {
        Self::Converted(format!("SQL error: {value}"))
    }
}

impl<'a> Controller<'a> {
    /// Creates a new [`Controller`]
    pub async fn new() -> Self {
        let pool = db::setup_conn()
            .await
            .expect("Failed to set up connection pool!");

        Self {
            pool,
            transaction: None,
        }
    }

    /// Runs the repl with this [`Controller`] as the parent
    ///
    /// Returns an [`sqlx::Error`] if `self.transaction` is `Some(t)` when the repl exits and the
    /// transaction fails to be rolled back
    pub async fn run_repl(mut self) -> Result<(), sqlx::Error> {
        repl::repl(&mut self).await;
        if let Some(t) = self.transaction {
            t.rollback().await?;
        }
        self.pool.close().await;
        Ok(())
    }

    /// Executes a [`Command`] on this controller
    ///
    /// # Parameters
    /// - `c` the [`Command`] to execute
    ///
    /// # Returns
    /// - Ok [`ControlResult`] if the execution succeeded
    /// - Err [`ControlError`] if the execution failed
    pub async fn execute(&mut self, c: Command) -> Result<ControlResult, ControlError> {
        match c {
            Command::Begin => self.begin().await,
            Command::Commit => self.commit().await,
            Command::Rent(u, i) => self.rent(&u, &i).await,
            Command::Rollback => self.rollback().await,
            Command::Terminate(id) => self.terminate(&id).await,
            Command::TryTerminate(u, i) => self.try_terminate(&u, &i).await,
            Command::List(o) => self.list(o).await,
        }
    }

    async fn begin(&mut self) -> Result<ControlResult, ControlError> {
        if let Some(t) = self.transaction.take() {
            t.rollback().await?;
            self.transaction = Some(self.pool.begin().await?);
        } else {
            self.transaction = Some(self.pool.begin().await?);
        }
        Ok(ControlResult::Begin)
    }

    async fn commit(&mut self) -> Result<ControlResult, ControlError> {
        self.transaction
            .take()
            .ok_or(ControlError::TransactionNone)?
            .commit()
            .await?;

        Ok(ControlResult::Commit)
    }

    async fn rollback(&mut self) -> Result<ControlResult, ControlError> {
        self.transaction
            .take()
            .ok_or(ControlError::TransactionNone)?
            .rollback()
            .await?;

        Ok(ControlResult::Rollback)
    }

    async fn rent(&mut self, user: &str, inst: &str) -> Result<ControlResult, ControlError> {
        let (u, i) = u_i_parse(user, inst)?;
        let tx = self.guard()?;

        db::lock_rentings(tx, u, i).await?;
        let max = db::get_max_rentals(tx).await?.parse::<i64>()?;
        let ur = db::count_user_rentals(tx, u).await?;

        if ur >= max {
            Err(ControlError::TooManyRentals)
        } else {
            Ok(ControlResult::Rent(db::rent(tx, u, i).await?))
        }
    }

    async fn try_terminate(
        &mut self,
        user: &str,
        inst: &str,
    ) -> Result<ControlResult, ControlError> {
        let (u, i) = u_i_parse(user, inst)?;
        let tx = self.guard()?;

        db::lock_rentings(tx, u, i).await?;
        let vec = db::find_to_terminate(tx, u, i).await?;

        match vec.len() {
            0 => Err(sqlx::Error::RowNotFound.into()),
            1 => Ok(ControlResult::TryTerminate(
                db::terminate_rid(tx, vec[0].get_id()).await?,
            )),
            _ => Err(ControlError::TerminateMultiple(vec)),
        }
    }

    async fn terminate(&mut self, id: &str) -> Result<ControlResult, ControlError> {
        let tx = self.guard()?;
        let i = id.parse::<i32>()?;
        Ok(ControlResult::Terminate(db::terminate_rid(tx, i).await?))
    }

    async fn list(&mut self, o: Option<String>) -> Result<ControlResult, ControlError> {
        let tx = self.guard()?;

        let rows = match o {
            Some(t) => db::list_type(tx, format!("{}%", t.to_lowercase())).await?,
            None => db::list_all(tx).await?,
        };

        if rows.is_empty() {
            return Err(sqlx::Error::RowNotFound.into());
        }

        let mut ret = vec![];
        for i in rows {
            let rent_count = db::count_instrument_rentals(tx, i.get_id()).await?;
            let available = i64::from(i.get_count()) - rent_count;
            if available > 0 {
                ret.push(i.to_string(available));
            }
        }
        Ok(ControlResult::List(ret))
    }

    fn guard<'b>(&'b mut self) -> Result<&'b mut Transaction<'a, Postgres>, ControlError> {
        self.transaction
            .as_mut()
            .ok_or(ControlError::TransactionNone)
    }
}

fn u_i_parse(u: &str, i: &str) -> Result<(i32, i32), ControlError> {
    Ok((u.parse::<i32>()?, i.parse::<i32>()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INSTRUMENT_ID: &str = "1";
    const TEST_RENT_ID: &str = "0";
    const TEST_STUDENT_ID: &str = "3";

    async fn init<'a>() -> Controller<'a> {
        let mut c = Controller::new().await;
        c.begin().await.unwrap();
        c
    }

    #[tokio::test]
    async fn test_rent_too_many() {
        let mut c = init().await;
        let max = db::get_max_rentals(c.transaction.as_mut().unwrap())
            .await
            .unwrap()
            .parse::<i64>()
            .unwrap();

        for _ in 0..max {
            let v = c.rent(TEST_STUDENT_ID, TEST_INSTRUMENT_ID).await;
            if v.is_err() {
                c.rollback().await.unwrap();
                panic!("Failed renting, wrong params for rent()?");
            } else {
                assert_eq!(v.unwrap(), ControlResult::Rent(1));
            }
        }

        let v = c.rent(TEST_STUDENT_ID, TEST_INSTRUMENT_ID).await;
        if v.is_ok() {
            c.rollback().await.unwrap();
            panic!("Renting should fail above max allowed")
        }

        assert_eq!(v.unwrap_err(), ControlError::TooManyRentals);
        c.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_terminate_multiple() {
        let mut c = init().await;

        for _ in 0..2 {
            let v = c.rent(TEST_STUDENT_ID, TEST_INSTRUMENT_ID).await;
            if v.is_err() {
                c.rollback().await.unwrap();
                panic!("Failed renting, wrong params for rent()?");
            } else {
                assert_eq!(v.unwrap(), ControlResult::Rent(1));
            }
        }

        let v = c.try_terminate(TEST_STUDENT_ID, TEST_INSTRUMENT_ID).await;
        if v.is_ok() {
            c.rollback().await.unwrap();
            panic!("Having mutliple possible terminations should return an error!")
        }

        assert!(matches!(v.unwrap_err(), ControlError::TerminateMultiple(_)));
        c.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_guard() {
        let mut c = Controller::new().await;
        let v = c.rent(TEST_STUDENT_ID, TEST_INSTRUMENT_ID).await;
        assert_eq!(v.unwrap_err(), ControlError::TransactionNone);
        let v = c.try_terminate(TEST_STUDENT_ID, TEST_INSTRUMENT_ID).await;
        assert_eq!(v.unwrap_err(), ControlError::TransactionNone);
        let v = c.terminate(TEST_RENT_ID).await;
        assert_eq!(v.unwrap_err(), ControlError::TransactionNone);
        let v = c.list(None).await;
        assert_eq!(v.unwrap_err(), ControlError::TransactionNone);
    }
}
