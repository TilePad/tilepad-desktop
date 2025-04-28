use super::{Migration, schema::*};
use sea_query::{IdenStatic, SqliteQueryBuilder, Table};

pub struct SettingsMigration;

#[async_trait::async_trait]
impl Migration for SettingsMigration {
    fn name(&self) -> &str {
        "m202504281419_create_settings_table"
    }

    async fn up(&self, db: &crate::database::DbPool) -> anyhow::Result<()> {
        sqlx::query(
            &Table::create()
                .table(SettingsTable)
                .if_not_exists()
                .col(pk_integer(SettingsColumn::Id))
                .col(json(SettingsColumn::Config))
                .build(SqliteQueryBuilder),
        )
        .execute(db)
        .await?;

        Ok(())
    }
}

#[derive(IdenStatic, Copy, Clone)]
#[iden(rename = "settings")]
pub struct SettingsTable;

#[derive(IdenStatic, Copy, Clone)]
pub enum SettingsColumn {
    /// ID for the row
    Id,
    /// settings configuration (JSON)
    Config,
}
