use crate::{
    database::{
        DbErr, DbPool, DbResult,
        helpers::{UpdateStatementExt, sql_exec, sql_query_maybe_one},
    },
    utils::device::get_device_name,
};
use sea_query::{Expr, IdenStatic, Query};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

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
}

impl Default for SettingsConfig {
    fn default() -> Self {
        let device_name = get_device_name();

        Self {
            language: "en".to_string(),
            start_automatically: false,
            device_name,
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
        sql_exec(
            db,
            Query::update()
                .table(SettingsTable)
                .and_where(Expr::col(SettingsColumn::Id).eq(self.id))
                .value_json(SettingsColumn::Config, &config)?,
        )
        .await?;

        self.config = config;

        Ok(self)
    }

    // Get a settings model by ID
    async fn get_by_id(db: &DbPool, id: u32) -> DbResult<Option<SettingsModel>> {
        sql_query_maybe_one(
            db,
            Query::select()
                .from(SettingsTable)
                .columns([SettingsColumn::Id, SettingsColumn::Config])
                .and_where(Expr::col(SettingsColumn::Id).eq(id)),
        )
        .await
    }

    /// Create a settings model
    async fn create(db: &DbPool, config: SettingsConfig) -> DbResult<SettingsModel> {
        let model = SettingsModel {
            id: Self::ID,
            config,
        };

        let config =
            serde_json::to_value(&model.config).map_err(|err| DbErr::Encode(err.into()))?;

        sql_exec(
            db,
            Query::insert()
                .into_table(SettingsTable)
                .columns([SettingsColumn::Id, SettingsColumn::Config])
                .values_panic([model.id.into(), config.into()]),
        )
        .await?;

        Ok(model)
    }
}
