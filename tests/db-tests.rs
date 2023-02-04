#[cfg(test)]
mod db_tests {
    use anyhow::Result;
    use dbexample_rs::*;

    #[sqlx::test(migrations = "db/migrations")]
    async fn create_test(pool: sqlx::MySqlPool) -> Result<()> {
        let names = ["Foo", "Bar", "Baz", "Hoge", "Fuga"];
        for name in names {
            let user = create_user(&pool, name).await?;
            assert_eq!(&user.name, name);
        }
        Ok(())
    }
}
