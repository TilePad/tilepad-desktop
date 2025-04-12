use crate::{
    database::{
        DbPool, DbResult, JsonObject,
        helpers::{sql_exec, sql_query_maybe_one},
    },
    plugin::manifest::PluginId,
};

use sea_query::{Expr, IdenStatic, OnConflict, Query};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PluginPropertiesModel {
    pub plugin_id: String,
    #[sqlx(json)]
    pub properties: JsonObject,
}

impl PluginPropertiesModel {
    /// Create a new profile
    pub async fn set(db: &DbPool, plugin_id: PluginId, properties: JsonObject) -> DbResult<()> {
        sql_exec(
            db,
            Query::insert()
                .into_table(PluginPropertiesTable)
                .columns([
                    PluginPropertiesColumn::PluginId,
                    PluginPropertiesColumn::Properties,
                ])
                .values_panic([
                    plugin_id.0.into(),
                    serde_json::Value::Object(properties).into(),
                ])
                .on_conflict(
                    OnConflict::column(PluginPropertiesColumn::PluginId)
                        .update_column(PluginPropertiesColumn::Properties)
                        .to_owned(),
                ),
        )
        .await?;

        Ok(())
    }

    pub async fn get_by_plugin(
        db: &DbPool,
        plugin_id: PluginId,
    ) -> DbResult<Option<PluginPropertiesModel>> {
        sql_query_maybe_one(
            db,
            Query::select()
                .from(PluginPropertiesTable)
                .columns([
                    PluginPropertiesColumn::PluginId,
                    PluginPropertiesColumn::Properties,
                ])
                .and_where(Expr::col(PluginPropertiesColumn::PluginId).eq(plugin_id.0)),
        )
        .await
    }

    #[allow(unused)]
    pub async fn delete(db: &DbPool, plugin_id: PluginId) -> DbResult<()> {
        sql_exec(
            db,
            Query::delete()
                .from_table(PluginPropertiesTable)
                .and_where(Expr::col(PluginPropertiesColumn::PluginId).eq(plugin_id.0)),
        )
        .await
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
