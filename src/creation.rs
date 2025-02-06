use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::ConnectOptions;
use std::str::FromStr;
use std::path::Path;
use sqlx::migrate::Migrator;

pub async fn create_encrypted_database(database_path: &str, password: &str) -> anyhow::Result<()> {
    let _ = SqliteConnectOptions::from_str(&database_path)?
        .pragma("key", password.to_owned())
        .create_if_missing(true)
        .connect()
        .await?;

    Ok(())
}

pub async fn run_migration_encrypted_database(database_path:&str, password:&str) -> anyhow::Result<()> {
    let db_path = Path::new(database_path);

    let m = Migrator::new(Path::new("./migrations")).await?;
    let pool = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::from_str(&db_path.to_str().unwrap())
                .unwrap()
                .pragma("key",password.to_owned())
                .create_if_missing(true),
        )
        .await?;

    m.run(&pool).await?;

    Ok(())
}

pub async fn run_embedded_migration_encrypted_database(database_path:&str, password:&str) -> anyhow::Result<()> {
    let db_path = Path::new(database_path);

    let pool = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::from_str(&db_path.to_str().unwrap())
                .unwrap()
                .pragma("key",password.to_owned())
                .create_if_missing(true),
        )
        .await?;

    // By default, the `migrate!` macro will search for sql files
    // in the migrations folder.
    // We need to change this behavior by setting the parameter
    // to the `encrypted_db/embedded_migrations` folder.
    sqlx::migrate!("encrypted_db/embedded_migrations")
        .run(&pool)
        .await?;

    Ok(())
}