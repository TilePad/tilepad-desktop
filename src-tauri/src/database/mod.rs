use anyhow::Context;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::{path::PathBuf, str::FromStr};
use tokio::fs::{File, create_dir_all};

pub mod entity;
mod migrations;

pub type DbPool = SqlitePool;
pub type DbErr = sqlx::Error;
pub type DbResult<T> = Result<T, DbErr>;
pub type JsonObject = serde_json::Map<String, serde_json::Value>;

/// Connects to the SQLite database at the provided path, creating a
/// new database file if none exist
pub async fn connect_database(path: PathBuf) -> anyhow::Result<DbPool> {
    if !path.exists() {
        let parent = path.parent().context("database path invalid")?;
        create_dir_all(parent)
            .await
            .context("create database path")?;

        File::create(&path).await?;
    }

    let path = path.to_str().context("invalid db path")?;
    let path = format!("sqlite://{path}");

    let options = SqliteConnectOptions::from_str(&path).context("failed to parse connection")?;
    let db = SqlitePool::connect_with(options)
        .await
        .context("failed to connect")?;

    migrations::migrate(&db)
        .await
        .context("failed to run database migrations")?;

    Ok(db)
}

#[cfg(test)]
#[allow(unused)]
pub async fn mock_database() -> DbPool {
    let db = SqlitePool::connect_with(SqliteConnectOptions::from_str("sqlite::memory:").unwrap())
        .await
        .unwrap();
    migrations::migrate(&db).await.unwrap();
    db
}
