use chrono::{DateTime, Utc};
use sea_query::{Expr, IdenStatic, Query};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::database::{
    helpers::{sql_exec, sql_query_all, sql_query_maybe_one},
    DbPool, DbResult,
};

use super::{folder::FolderId, profile::ProfileId};

pub type DeviceId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeviceModel {
    pub id: DeviceId,
    pub name: String,
    /// Access token should not be serialized
    #[serde(skip)]
    pub access_token: String,
    #[sqlx(json)]
    pub config: DeviceConfig,
    pub order: u32,
    pub created_at: DateTime<Utc>,
    pub last_connected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub profile_id: ProfileId,
    pub folder_id: FolderId,
}

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
                    model.created_at.into(),
                    model.last_connected_at.into(),
                ]),
        )
        .await?;

        Ok(model)
    }

    pub async fn set_connected_now(&mut self, db: &DbPool) -> DbResult<()> {
        self.last_connected_at = Utc::now();
        sql_exec(
            db,
            Query::update()
                .table(DevicesTable)
                .value(
                    DevicesColumn::LastConnectedAt,
                    Expr::value(self.last_connected_at),
                )
                .and_where(Expr::col(DevicesColumn::Id).eq(self.id)),
        )
        .await
    }

    /// Get a device using its access token
    pub async fn get_by_access_token(
        db: &DbPool,
        access_token: &str,
    ) -> DbResult<Option<DeviceModel>> {
        sql_query_maybe_one(
            db,
            Query::select()
                .from(DevicesTable)
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
                .and_where(Expr::col(DevicesColumn::AccessToken).eq(access_token)),
        )
        .await
    }

    pub async fn get_by_id(db: &DbPool, id: DeviceId) -> DbResult<Option<DeviceModel>> {
        sql_query_maybe_one(
            db,
            Query::select()
                .from(DevicesTable)
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
                .and_where(Expr::col(DevicesColumn::Id).eq(id)),
        )
        .await
    }

    pub async fn all(db: &DbPool) -> DbResult<Vec<DeviceModel>> {
        sql_query_all(
            db,
            Query::select().from(DevicesTable).columns([
                DevicesColumn::Id,
                DevicesColumn::Name,
                DevicesColumn::AccessToken,
                DevicesColumn::Config,
                DevicesColumn::Order,
                DevicesColumn::ProfileId,
                DevicesColumn::CreatedAt,
                DevicesColumn::LastConnectedAt,
            ]),
        )
        .await
    }

    pub async fn delete(db: &DbPool, device_id: DeviceId) -> DbResult<()> {
        sql_exec(
            db,
            Query::delete()
                .from_table(DevicesTable)
                .and_where(Expr::col(DevicesColumn::Id).eq(device_id)),
        )
        .await
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
