use std::env;
use std::env::VarError;

use anyhow::{bail, Result};
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=migrations");

    dotenv::dotenv().ok();
    match env::var("DATABASE_URL") {
        Ok(database_url) => {
            let pool = Pool::<Postgres>::connect(&database_url).await?;
            sqlx::migrate!().run(&pool).await?;
        }
        Err(VarError::NotPresent) => {
            println!("cargo:warning=DATABASE_URL env var not set. Skipping migrations.");
        }
        Err(error) => {
            bail!("Unable to read DATABASE_URL env var: {error:?}");
        }
    }

    Ok(())
}
