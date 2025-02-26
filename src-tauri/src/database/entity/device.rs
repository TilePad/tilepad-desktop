use chrono::{DateTime, Utc};
use sea_query::{IdenStatic, Query};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::database::{helpers::sql_exec, DbPool, DbResult};

use super::profile::ProfileId;

pub type DeviceId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeviceModel {
    pub id: DeviceId,
    pub name: String,
    pub access_token: String,
    #[sqlx(json)]
    pub config: DeviceConfig,
    pub order: u32,
    pub profile_id: Option<ProfileId>,
    pub created_at: DateTime<Utc>,
    pub last_connected_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {}

pub struct CreateDevice {
    pub name: String,
    pub access_token: String,
    pub config: DeviceConfig,
}

impl DeviceModel {
    pub async fn create(db: &DbPool, create: CreateDevice) -> anyhow::Result<DeviceModel> {
        let model = DeviceModel {
            id: Uuid::new_v4(),
            name: create.name,
            access_token: create.access_token,
            config: create.config,
            order: 0,
            profile_id: None,
            created_at: Utc::now(),
            last_connected_at: Utc::now(),
        };

        let config = serde_json::to_value(&model.config)?;

        sql_exec(
            db,
            Query::insert()
                .into_table(DevicesTable)
                .columns([
                    DevicesColumn::Id,
                    DevicesColumn::Name,
                    DevicesColumn::AccessToken,
                    DevicesColumn::Config,
                    DevicesColumn::Order,
                    DevicesColumn::ProfileId,
                    DevicesColumn::CreatedAt,
                    DevicesColumn::LastConnectedAt,
                ])
                .values_panic([
                    model.id.into(),
                    model.name.clone().into(),
                    model.access_token.clone().into(),
                    config.into(),
                    model.order.into(),
                    model.profile_id.into(),
                    model.created_at.into(),
                    model.last_connected_at.into(),
                ]),
        )
        .await?;

        Ok(model)
    }
}

#[derive(IdenStatic, Copy, Clone)]
#[iden(rename = "devices")]
pub struct DevicesTable;

#[derive(IdenStatic, Copy, Clone)]
pub enum DevicesColumn {
    /// Unique ID for the device
    Id,
    /// Name of the device
    Name,
    /// Access token for the device
    AccessToken,
    /// Additional device configuration
    Config,
    /// Order of the device in the UI
    Order,
    /// Current profile on the device
    ProfileId,
    /// Timestamp of when the device was first approved
    CreatedAt,
    /// Timestamp of when the device last connected
    LastConnectedAt,
}
