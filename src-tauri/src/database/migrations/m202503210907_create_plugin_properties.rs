use super::{Migration, schema::*};
use sea_query::{IdenStatic, SqliteQueryBuilder, Table};

pub struct PluginPropertiesMigration;

#[async_trait::async_trait]
impl Migration for PluginPropertiesMigration {
    fn name(&self) -> &str {
        "m202503210907_create_plugin_properties"
    }

    async fn up(&self, db: &crate::database::DbPool) -> anyhow::Result<()> {
        sqlx::query(
            &Table::create()
                .table(PluginPropertiesTable)
                .if_not_exists()
                .col(string(PluginPropertiesColumn::PluginId).primary_key())
                .col(json(PluginPropertiesColumn::Properties))
                .build(SqliteQueryBuilder),
        )
        .execute(db)
        .await?;

        Ok(())
    }
}

#[derive(IdenStatic, Copy, Clone)]
#[iden(rename = "plugin_properties")]
pub struct PluginPropertiesTable;

#[derive(IdenStatic, Copy, Clone)]
pub enum PluginPropertiesColumn {
    /// ID of
    PluginId,
    /// Properties (JSON)
    Properties,
}
