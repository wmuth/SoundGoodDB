use controller::Controller;

mod controller;
mod db;
mod parser;
mod repl;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let con = Controller::new().await;
    con.run_repl().await?;
    Ok(())
}
