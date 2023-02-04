use std::env;

use anyhow::Result;
use sqlx::mysql::MySqlPool;

use dbexample_rs::{create_user, read_all_users};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    create_user(&pool, "H1rono").await?;
    let users = read_all_users(&pool).await?;
    println!("{users:?}");
    Ok(())
}
