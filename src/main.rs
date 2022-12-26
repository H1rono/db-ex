use std::env;

use anyhow::Result;
use sqlx::mysql::MySqlPool;

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::query!(r"INSERT INTO `users` (`name`) VALUES ('H1rono')")
        .execute(&pool)
        .await?;
    let users = sqlx::query_as!(User, r"SELECT `id`, `name` FROM `users`")
        .fetch_all(&pool)
        .await?;
    for user in users {
        println!("{:?}", user);
    }
    Ok(())
}
