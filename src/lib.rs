use anyhow::{Context, Result};

use sqlx::FromRow;

#[derive(Debug, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: u32,
    pub name: String,
}

pub async fn read_all_users(pool: &sqlx::MySqlPool) -> Result<Vec<User>> {
    let res = sqlx::query("SELECT `id`, `name` FROM `users`")
        .fetch_all(pool)
        .await
        .context("Failed to SELECT all users")?;
    let users = res
        .into_iter()
        .map(|u| User::from_row(&u).with_context(|| format!("Failed to parse row {u:?}")))
        .collect::<Result<Vec<User>>>()?;
    Ok(users)
}

pub async fn create_user(pool: &sqlx::MySqlPool, name: &str) -> Result<User> {
    let res = sqlx::query("INSERT `users` (`name`) VALUES (?)")
        .bind(name)
        .execute(pool)
        .await
        .with_context(|| format!("Failed to INSERT a new user with name='{name}'"))?;
    let user = User {
        id: res.last_insert_id() as u32,
        name: String::from(name),
    };
    Ok(user)
}

pub async fn update_user(pool: &sqlx::MySqlPool, id: u32, name: &str) -> Result<User> {
    sqlx::query("UPDATE `users` SET `name` = ? WHERE `id` = ?")
        .bind(name)
        .bind(id)
        .execute(pool)
        .await
        .with_context(|| format!("Failed to UPDATE user name='{name}' WHERE id='{id}'"))?;
    let user = User {
        id,
        name: String::from(name),
    };
    Ok(user)
}

pub async fn delete_user(pool: &sqlx::MySqlPool, id: u32) -> Result<()> {
    sqlx::query("DELETE FROM `users` WHERE `id` = ?")
        .bind(id)
        .execute(pool)
        .await
        .with_context(|| format!("Failed to DELETE user WHERE id='{id}'"))?;
    Ok(())
}
