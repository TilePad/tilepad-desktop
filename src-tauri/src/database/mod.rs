use anyhow::Context;
use log::warn;
use migration::Migrator;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::path::PathBuf;
use tokio::fs::{create_dir_all, File};

pub mod entity;
mod migration;

/// Connects to the SQLite database at the provided path, creating a
/// new database file if none exist
pub async fn connect_database(path: PathBuf) -> anyhow::Result<DatabaseConnection> {
    if !path.exists() {
        let parent = path.parent().context("database path invalid")?;
        create_dir_all(parent)
            .await
            .context("create database path")?;

        File::create(&path).await?;
    }

    let path = path.to_str().context("invalid db path")?;

    let path = format!("sqlite://{path}");

    let options = sea_orm::ConnectOptions::new(path);
    let db = Database::connect(options).await?;

    setup_database(&db).await?;

    Ok(db)
}

#[cfg(test)]
pub async fn mock_database() -> DatabaseConnection {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    setup_database(&db).await.unwrap();
    db
}

pub async fn setup_database(db: &DatabaseConnection) -> anyhow::Result<()> {
    if let Err(err) = Migrator::up(db, None).await {
        warn!("failed to apply/check database migrations: {:?}", err);
        // TODO: Check for applied forward migrations, these are not always failing changes
    }

    Ok(())
}
