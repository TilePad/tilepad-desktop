use crate::{
    database::{DbErr, DbPool, DbResult},
    utils::device::get_device_name,
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SettingsModel {
    pub id: u32,
    #[sqlx(json)]
    pub config: SettingsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SettingsConfig {
    pub language: String,
    pub start_automatically: bool,
    pub device_name: String,
    pub developer_mode: bool,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        let device_name = get_device_name();

        Self {
            language: "en".to_string(),
            start_automatically: false,
            device_name,
            developer_mode: false,
        }
    }
}

impl SettingsModel {
    /// Settings table is a singleton so we always use the same ID
    const ID: u32 = 1;

    // Get the settings model or a default value
    pub async fn get_or_default(db: &DbPool) -> DbResult<SettingsModel> {
        if let Some(model) = Self::get_by_id(db, Self::ID).await? {
            return Ok(model);
        }

        Self::create(db, SettingsConfig::default()).await
    }

    pub async fn update(mut self, db: &DbPool, config: SettingsConfig) -> DbResult<SettingsModel> {
        let config_json = serde_json::to_value(&config).map_err(|err| DbErr::Encode(err.into()))?;

        sqlx::query("UPDATE \"settings\" SET \"config\" = ? WHERE \"id\" = ?")
            .bind(config_json)
            .bind(self.id)
            .execute(db)
            .await?;

        self.config = config;

        Ok(self)
    }

    // Get a settings model by ID
    async fn get_by_id(db: &DbPool, id: u32) -> DbResult<Option<SettingsModel>> {
        sqlx::query_as("SELECT * FROM \"settings\" WHERE \"id\" = ?")
            .bind(id)
            .fetch_optional(db)
            .await
    }

    /// Create a settings model
    async fn create(db: &DbPool, config: SettingsConfig) -> DbResult<SettingsModel> {
        let model = SettingsModel {
            id: Self::ID,
            config,
        };

        let config_json =
            serde_json::to_value(&model.config).map_err(|err| DbErr::Encode(err.into()))?;

        sqlx::query(
            "
                INSERT INTO \"settings\" (\"id\", \"config\")
                VALUES (?, ?)
            ",
        )
        .bind(model.id)
        .bind(config_json)
        .execute(db)
        .await?;

        Ok(model)
    }
}
