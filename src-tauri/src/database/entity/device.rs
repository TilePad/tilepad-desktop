use chrono::{DateTime, Utc};
use sea_query::{Expr, IdenStatic, Query};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::database::{
    helpers::{sql_exec, sql_query_all, sql_query_maybe_one, UpdateStatementExt},
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
    pub profile_id: ProfileId,
    pub folder_id: FolderId,
    pub order: u32,
    pub created_at: DateTime<Utc>,
    pub last_connected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {}

pub struct CreateDevice {
    pub name: String,
    pub access_token: String,
    pub config: DeviceConfig,
    pub profile_id: ProfileId,
    pub folder_id: FolderId,
}

#[derive(Default, Deserialize)]
pub struct UpdateDevice {
    pub name: Option<String>,
    pub config: Option<DeviceConfig>,
    pub order: Option<u32>,
    pub profile_id: Option<ProfileId>,
    pub folder_id: Option<FolderId>,
}

impl DeviceModel {
    pub async fn create(db: &DbPool, create: CreateDevice) -> anyhow::Result<DeviceModel> {
        let model = DeviceModel {
            id: Uuid::new_v4(),
            name: create.name,
            access_token: create.access_token,
            config: create.config,
            order: 0,
            profile_id: create.profile_id,
            folder_id: create.folder_id,
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
                    DevicesColumn::FolderId,
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
                    model.folder_id.into(),
                    model.created_at.into(),
                    model.last_connected_at.into(),
                ]),
        )
        .await?;

        Ok(model)
    }

    pub async fn update(
        mut self,
        db: &DbPool,
        update: UpdateDevice,
    ) -> anyhow::Result<DeviceModel> {
        sql_exec(
            db,
            Query::update()
                .table(DevicesTable)
                .and_where(Expr::col(DevicesColumn::Id).eq(self.id))
                .cond_value(DevicesColumn::Name, update.name.as_ref())
                .cond_value_json(DevicesColumn::Config, update.config.as_ref())?
                .cond_value(DevicesColumn::Order, update.order)
                .cond_value(DevicesColumn::ProfileId, update.profile_id)
                .cond_value(DevicesColumn::FolderId, update.folder_id),
        )
        .await?;

        self.name = update.name.unwrap_or(self.name);
        self.config = update.config.unwrap_or(self.config);
        self.order = update.order.unwrap_or(self.order);
        self.profile_id = update.profile_id.unwrap_or(self.profile_id);
        self.folder_id = update.folder_id.unwrap_or(self.folder_id);

        Ok(self)
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
                    DevicesColumn::FolderId,
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
                    DevicesColumn::FolderId,
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
                DevicesColumn::FolderId,
                DevicesColumn::CreatedAt,
                DevicesColumn::LastConnectedAt,
            ]),
        )
        .await
    }

    pub async fn all_by_profile(db: &DbPool, profile_id: ProfileId) -> DbResult<Vec<DeviceModel>> {
        sql_query_all(
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
                    DevicesColumn::FolderId,
                    DevicesColumn::CreatedAt,
                    DevicesColumn::LastConnectedAt,
                ])
                .and_where(Expr::col(DevicesColumn::ProfileId).eq(profile_id)),
        )
        .await
    }

    pub async fn all_by_folder(db: &DbPool, folder_id: FolderId) -> DbResult<Vec<DeviceModel>> {
        sql_query_all(
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
                    DevicesColumn::FolderId,
                    DevicesColumn::CreatedAt,
                    DevicesColumn::LastConnectedAt,
                ])
                .and_where(Expr::col(DevicesColumn::FolderId).eq(folder_id)),
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
    /// Current profile the device is using
    ProfileId,
    /// Current folder the device is using
    FolderId,
    /// Timestamp of when the device was first approved
    CreatedAt,
    /// Timestamp of when the device last connected
    LastConnectedAt,
}
