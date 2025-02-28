use anyhow::Context;
use entity::profile::{CreateProfile, ProfileConfig, ProfileModel};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::{path::PathBuf, str::FromStr};
use tokio::fs::{create_dir_all, File};

pub mod entity;
pub mod helpers;
mod migrations;

pub type DbPool = SqlitePool;
pub type DbErr = sqlx::Error;
pub type DbResult<T> = Result<T, DbErr>;

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

    setup_database(&db).await.context("failed to setup")?;

    Ok(db)
}

#[cfg(test)]
pub async fn mock_database() -> DbPool {
    let db = SqlitePool::connect_with(SqliteConnectOptions::from_str("sqlite::memory:").unwrap())
        .await
        .unwrap();

    setup_database(&db).await.unwrap();
    db
}

pub async fn setup_database(db: &DbPool) -> anyhow::Result<()> {
    migrations::migrate(db)
        .await
        .context("failed to migrate database")?;

    try_create_default_profile(db).await?;

    Ok(())
}

pub async fn try_create_default_profile(db: &DbPool) -> anyhow::Result<()> {
    // Default profile already exists
    if ProfileModel::get_default_profile(db).await?.is_some() {
        return Ok(());
    }

    // Create default profile
    let profile = ProfileModel::create(
        db,
        CreateProfile {
            default: true,
            name: "Default Profile".to_string(),
            config: ProfileConfig::default(),
            order: 0,
        },
    )
    .await?;

    Ok(())
}
