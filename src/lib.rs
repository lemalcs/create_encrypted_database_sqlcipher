use sqlx::sqlite::{SqliteConnectOptions, SqliteQueryResult};
use sqlx::{query, ConnectOptions, Connection, SqliteConnection};

use std::str::FromStr;
use std::path::Path;
use std::fs;
mod creation;
use crate::creation::*;

async fn fill_db(conn: &mut SqliteConnection) -> anyhow::Result<SqliteQueryResult> {
    conn.transaction(|tx| {
        Box::pin(async move {
            query(
                "
            CREATE TABLE Tool(
                Id INT PRIMARY KEY     NOT NULL,
                Name           TEXT    NOT NULL,
                Description    TEXT    NOT NULL,
                Price         REAL
             );
             ",
            )
                .execute(&mut **tx)
                .await?;

            query(
                r#"
            INSERT INTO Tool(Id, Name, Description, Price)
            VALUES
                (1, "Hammer", "🔨", 0.1),
                (2, "Screwdriver","🪛", 2.6),
                (3, "Wrench","🔧", 1.4)
            "#,
            )
                .execute(&mut **tx)
                .await
        })
    })
        .await
        .map_err(|e| e.into())
}

#[tokio::test]
async fn test_create_encrypted_database() {
    let database_path = "C:\\store\\myencryptedatabase.sqlite3";
    let password = "my_top_secret_passworD";
    let _ = create_encrypted_database(&database_path, password).await;

    let mut conn = SqliteConnectOptions::from_str(&database_path)
        .unwrap()
        .pragma("key", password)
        .create_if_missing(true)
        .connect()
        .await
        .unwrap();

    let _ = fill_db(&mut conn).await;

    // Create another connection without a password, the query should fail
    let mut conn = SqliteConnectOptions::from_str(&database_path)
        .unwrap()
        .connect()
        .await
        .unwrap();

    assert!(conn
        .transaction(|tx| {
            Box::pin(async move { query("SELECT * FROM Tool;").fetch_all(&mut **tx).await })
        })
        .await
        .is_err());
}

#[tokio::test]
async fn test_run_migration_encrypted_sqlcipher() {
    let database_path = "C:\\store\\myprotectedatabase.sqlite3";
    let password = "my_top_secret_passworD";

    let _ = create_encrypted_database(&database_path, password).await;

    let mut conn = SqliteConnectOptions::from_str(&database_path)
        .unwrap()
        .pragma("key", password)
        .create_if_missing(true)
        .connect()
        .await
        .unwrap();

    let _ = fill_db(&mut conn).await;

    let result = run_migration_encrypted_database(&database_path, password).await;

    // Uncomment the line below to check any error message if the test fails
    //dbg!(&result);

    assert!(result.is_ok())
}

#[tokio::test]
async fn test_run_embedded_migration_encrypted_sqlcipher() {
    let database_path = "C:\\store\\myotherprotectedatabase.sqlite3";
    let password = "my_top_secret_passworD";

    if Path::new(database_path).exists(){
        fs::remove_file(database_path).unwrap();
    }

    let _ = create_encrypted_database(&database_path, password).await;

    let result=run_embedded_migration_encrypted_database(&database_path, password).await;

    //dbg!(&result);

    assert!(result.is_ok())
}