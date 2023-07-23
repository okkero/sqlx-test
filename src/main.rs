use std::env;

use anyhow::{anyhow, bail, Context, Result};
use sqlx::Pool;

#[derive(Debug)]
#[allow(unused)]
struct Foo {
    name: String,
    magic_number: i32,
    magic_text: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let mut args = env::args().skip(1);
    let cmd = args.next().ok_or(anyhow!("Please supply cmd"))?;

    let pool = Pool::connect(&env::var("DATABASE_URL")?).await?;

    match cmd.as_str() {
        "list" => {
            let foos = sqlx::query_file_as!(Foo, "src/sql/get_foo.sql")
                .fetch_all(&pool)
                .await?;

            println!("{foos:#?}");
        }
        "insert" => {
            let name = args.next().ok_or(anyhow!("Please supply name"))?;
            let magic_number = args
                .next()
                .ok_or(anyhow!("Please supply magic number"))?
                .parse::<i32>()
                .context("Invalid number")?;
            sqlx::query_file!("src/sql/insert_foo.sql", name, magic_number)
                .execute(&pool)
                .await?;
        }
        _ => bail!("Invalid cmd"),
    }

    sqlx::query!("insert into foo (name, magic_number, magic_text) values ('a', 42, null)")
        .execute(&pool)
        .await?;

    Ok(())
}
