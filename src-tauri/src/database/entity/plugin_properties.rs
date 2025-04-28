use crate::{
    database::{DbPool, DbResult, JsonObject},
    plugin::manifest::PluginId,
};

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
        sqlx::query(
            "
            INSERT INTO \"plugin_properties\" (\"plugin_id\", \"properties\")
            VALUES (?, ?)
            ON CONFLICT (\"plugin_id\")
            DO UPDATE SET \"properties\" = excluded.\"properties\"
        ",
        )
        .bind(plugin_id.0)
        .bind(serde_json::Value::Object(properties))
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn get_by_plugin(
        db: &DbPool,
        plugin_id: PluginId,
    ) -> DbResult<Option<PluginPropertiesModel>> {
        sqlx::query_as("SELECT * FROM \"plugin_properties\" WHERE \"plugin_id\" = ?")
            .bind(plugin_id.0)
            .fetch_optional(db)
            .await
    }

    #[allow(unused)]
    pub async fn delete(db: &DbPool, plugin_id: PluginId) -> DbResult<()> {
        sqlx::query("DELETE FROM \"plugin_properties\" WHERE \"plugin_id\" = ?")
            .bind(plugin_id.0)
            .execute(db)
            .await?;
        Ok(())
    }
}
