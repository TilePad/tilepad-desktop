use anyhow::Context;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use super::{DbPool, DbResult};

fn migrations() -> Vec<SqlMigration> {
    vec![
        SqlMigration::new(
            "m202502251151_create_profiles_table",
            include_str!("m202502251151_create_profiles_table.sql"),
        ),
        SqlMigration::new(
            "m202502251153_create_folders_table",
            include_str!("m202502251153_create_folders_table.sql"),
        ),
        SqlMigration::new(
            "m202502251225_create_tiles_table",
            include_str!("m202502251225_create_tiles_table.sql"),
        ),
        SqlMigration::new(
            "m202502251226_create_devices_table",
            include_str!("m202502251226_create_devices_table.sql"),
        ),
        SqlMigration::new(
            "m202503210907_create_plugin_properties",
            include_str!("m202503210907_create_plugin_properties.sql"),
        ),
        SqlMigration::new(
            "m202504281419_create_settings_table",
            include_str!("m202504281419_create_settings_table.sql"),
        ),
    ]
}

pub trait Migration: Send + Sync {
    fn name(&self) -> &str;

    async fn up(&self, db: &DbPool) -> DbResult<()>;
}

pub struct SqlMigration {
    name: &'static str,
    sql: &'static str,
}

impl SqlMigration {
    pub fn new(name: &'static str, sql: &'static str) -> Self {
        Self { name, sql }
    }
}

impl Migration for SqlMigration {
    fn name(&self) -> &str {
        self.name
    }

    async fn up(&self, db: &DbPool) -> DbResult<()> {
        sqlx::query(self.sql).execute(db).await?;
        Ok(())
    }
}

#[derive(FromRow)]
struct AppliedMigration {
    name: String,
    #[allow(unused)]
    applied_at: DateTime<Utc>,
}

pub async fn migrate(db: &DbPool) -> anyhow::Result<()> {
    create_migrations_table(db)
        .await
        .context("failed to create migrations table")?;

    let migrations = migrations();
    let mut applied = get_applied_migrations(db)
        .await
        .context("failed to get applied migrations")?;
    let mut migration_names = Vec::new();

    for migration in &migrations {
        let name = migration.name();
        migration_names.push(name.to_string());

        // Migration already applied
        if applied.iter().any(|applied| applied.name.eq(name)) {
            continue;
        }

        // Apply migration
        migration
            .up(db)
            .await
            .with_context(|| format!("failed to apply migration \"{name}\""))?;

        // Store applied migration
        let applied_at = Utc::now();
        let migration = create_applied_migration(db, name.to_string(), applied_at)
            .await
            .with_context(|| format!("failed to store applied migration \"{name}\""))?;

        applied.push(migration);
    }

    // Check if a migration was applied but is not known locally (warning)
    for applied in applied {
        if !migration_names.contains(&applied.name) {
            tracing::warn!(
                name = applied.name,
                "database has migration applied that is not known locally",
            );
        }
    }

    Ok(())
}

async fn create_migrations_table(db: &DbPool) -> DbResult<()> {
    sqlx::query(include_str!("m202502251151_create_migrations_table.sql"))
        .execute(db)
        .await?;
    Ok(())
}

async fn get_applied_migrations(db: &DbPool) -> DbResult<Vec<AppliedMigration>> {
    let result: Vec<AppliedMigration> = sqlx::query_as("SELECT * FROM migrations")
        .fetch_all(db)
        .await?;
    Ok(result)
}

async fn create_applied_migration(
    db: &DbPool,
    name: String,
    applied_at: DateTime<Utc>,
) -> DbResult<AppliedMigration> {
    sqlx::query("INSERT INTO migrations (name, applied_at) VALUES (?, ?)")
        .bind(&name)
        .bind(applied_at)
        .execute(db)
        .await?;

    let model = AppliedMigration { name, applied_at };
    Ok(model)
}
