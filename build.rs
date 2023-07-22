use std::env;

use anyhow::{Context, Result};
use sqlx::Postgres;

#[tokio::main]
async fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=migrations");

    dotenv::dotenv().ok();
    let database_url = &env::var("DATABASE_URL").context("DATABASE_URL not set")?;

    let pool = sqlx::pool::Pool::<Postgres>::connect(database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}
