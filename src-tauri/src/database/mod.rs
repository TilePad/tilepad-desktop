use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::{path::PathBuf, str::FromStr};
use thiserror::Error;
use tokio::fs::{File, create_dir_all};

pub mod entity;
mod migrations;

pub type DbPool = SqlitePool;
pub type DbErr = sqlx::Error;
pub type DbResult<T> = Result<T, DbErr>;
pub type JsonObject = serde_json::Map<String, serde_json::Value>;

#[derive(Debug, Error)]
pub enum DbConnectError {
    #[error(transparent)]
    Db(#[from] DbErr),
    #[error("invalid database file path")]
    InvalidPath,
    #[error("failed to create database file path: {0}")]
    CreatePath(std::io::Error),
    #[error("failed to create database file: {0}")]
    CreateFile(std::io::Error),
}

/// Connects to the SQLite database at the provided path, creating a
/// new database file if none exist
pub async fn connect_database(path: PathBuf) -> Result<DbPool, DbConnectError> {
    if !path.exists() {
        let parent = path.parent().ok_or(DbConnectError::InvalidPath)?;

        create_dir_all(parent)
            .await
            .map_err(DbConnectError::CreatePath)?;

        File::create(&path)
            .await
            .map_err(DbConnectError::CreateFile)?;
    }

    let path = path.to_str().ok_or(DbConnectError::InvalidPath)?;
    let path = format!("sqlite://{path}");

    let options = SqliteConnectOptions::from_str(&path)?;
    let db = SqlitePool::connect_with(options).await?;

    if let Err(cause) = migrations::migrate(&db).await {
        tracing::error!(?cause, "failed to run database migrations");
        return Err(cause.into());
    }

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
