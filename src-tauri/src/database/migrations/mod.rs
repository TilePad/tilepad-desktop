use anyhow::Context;
use chrono::{DateTime, Utc};
use log::warn;
use sea_query::{ColumnDef, IdenStatic, Query, SqliteQueryBuilder, Table};
use sea_query_binder::SqlxBinder;
use sqlx::prelude::FromRow;

use super::{DbPool, DbResult};

mod m202502251151_create_profiles_table;
mod m202502251225_create_tiles_table;
mod m202502251226_create_devices_table;
mod schema;

fn migrations() -> Vec<Box<dyn Migration>> {
    vec![Box::new(
        m202502251151_create_profiles_table::ProfilesMigration,
    )]
}

#[async_trait::async_trait]
pub trait Migration {
    fn name(&self) -> &str;

    async fn up(&self, db: &DbPool) -> anyhow::Result<()>;
}

#[derive(FromRow)]
struct AppliedMigration {
    name: String,
    #[allow(unused)]
    applied_at: DateTime<Utc>,
}

/// Table for storing migrations
#[derive(IdenStatic, Copy, Clone)]
#[iden(rename = "migrations")]
pub struct MigrationsTable;

/// Columns on the migrations table
#[derive(IdenStatic, Copy, Clone)]
pub enum MigrationsColumn {
    /// Name of the migration
    Name,
    /// When the migration was applied
    AppliedAt,
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
            warn!(
                "database has migration applied that is not known locally: \"{}\"",
                &applied.name
            );
        }
    }

    Ok(())
}

async fn create_migrations_table(db: &DbPool) -> anyhow::Result<()> {
    sqlx::query(
        &Table::create()
            .table(MigrationsTable)
            .if_not_exists()
            .col(
                ColumnDef::new(MigrationsColumn::Name)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(MigrationsColumn::AppliedAt)
                    .date_time()
                    .not_null(),
            )
            .build(SqliteQueryBuilder),
    )
    .execute(db)
    .await?;

    Ok(())
}

async fn get_applied_migrations(db: &DbPool) -> DbResult<Vec<AppliedMigration>> {
    let (query, _values) = Query::select()
        .columns([MigrationsColumn::Name, MigrationsColumn::AppliedAt])
        .from(MigrationsTable)
        .build(SqliteQueryBuilder);
    let result: Vec<AppliedMigration> = sqlx::query_as(&query).fetch_all(db).await?;
    Ok(result)
}

async fn create_applied_migration(
    db: &DbPool,
    name: String,
    applied_at: DateTime<Utc>,
) -> DbResult<AppliedMigration> {
    let (query, values) = Query::insert()
        .columns([MigrationsColumn::Name, MigrationsColumn::AppliedAt])
        .into_table(MigrationsTable)
        .values_panic([name.as_str().into(), applied_at.into()])
        .build_sqlx(SqliteQueryBuilder);

    sqlx::query_with(&query, values).execute(db).await?;

    let model = AppliedMigration { name, applied_at };

    Ok(model)
}
