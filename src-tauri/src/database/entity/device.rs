use super::{folder::FolderId, profile::ProfileId};
use crate::database::{DbErr, DbPool, DbResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

pub type DeviceId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeviceModel {
    pub id: DeviceId,

    /// Name of the device
    pub name: String,

    /// Client public key
    pub public_key: Vec<u8>,

    /// Device configuration
    #[sqlx(json)]
    pub config: DeviceConfig,

    /// Active device profile
    pub profile_id: ProfileId,

    /// Active device folder
    pub folder_id: FolderId,

    pub order: u32,
    pub created_at: DateTime<Utc>,
    pub last_connected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {}

pub struct CreateDevice {
    pub name: String,
    pub public_key: Vec<u8>,
    pub config: DeviceConfig,
    pub profile_id: ProfileId,
    pub folder_id: FolderId,
}

impl DeviceModel {
    pub async fn create(db: &DbPool, create: CreateDevice) -> DbResult<DeviceModel> {
        let model = DeviceModel {
            id: Uuid::new_v4(),
            name: create.name,
            public_key: create.public_key,
            config: create.config,
            order: 0,
            profile_id: create.profile_id,
            folder_id: create.folder_id,
            created_at: Utc::now(),
            last_connected_at: Utc::now(),
        };

        let config =
            serde_json::to_value(&model.config).map_err(|err| DbErr::Encode(err.into()))?;

        sqlx::query(
            r#"
            INSERT INTO "devices" (
                "id", 
                "name", 
                "public_key", 
                "config", 
                "order", 
                "profile_id", 
                "folder_id", 
                "created_at",
                "last_connected_at"
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        )
        .bind(model.id)
        .bind(model.name.clone())
        .bind(model.public_key.as_slice())
        .bind(config)
        .bind(model.order)
        .bind(model.profile_id)
        .bind(model.folder_id)
        .bind(model.created_at)
        .bind(model.last_connected_at)
        .execute(db)
        .await?;

        Ok(model)
    }

    pub async fn set_profile(
        mut self,
        db: &DbPool,
        profile_id: ProfileId,
        folder_id: FolderId,
    ) -> DbResult<DeviceModel> {
        sqlx::query(r#"UPDATE "devices" SET "profile_id" = ?, "folder_id" = ? WHERE "id" = ?"#)
            .bind(profile_id)
            .bind(folder_id)
            .bind(self.id)
            .execute(db)
            .await?;

        self.profile_id = profile_id;
        self.folder_id = folder_id;

        Ok(self)
    }

    pub async fn set_connected_now(&mut self, db: &DbPool) -> DbResult<()> {
        let last_connected_at = Utc::now();
        sqlx::query(r#"UPDATE "devices" SET "last_connected_at" = ? WHERE "id" = ?"#)
            .bind(last_connected_at)
            .bind(self.id)
            .execute(db)
            .await?;

        self.last_connected_at = last_connected_at;
        Ok(())
    }

    /// Get a device using its public key
    pub async fn get_by_public_key(
        db: &DbPool,
        public_key: &[u8],
    ) -> DbResult<Option<DeviceModel>> {
        sqlx::query_as(r#"SELECT * FROM "devices" WHERE "public_key" = ?"#)
            .bind(public_key)
            .fetch_optional(db)
            .await
    }

    pub async fn get_by_id(db: &DbPool, id: DeviceId) -> DbResult<Option<DeviceModel>> {
        sqlx::query_as(r#"SELECT * FROM "devices" WHERE "id" = ?"#)
            .bind(id)
            .fetch_optional(db)
            .await
    }

    pub async fn all(db: &DbPool) -> DbResult<Vec<DeviceModel>> {
        sqlx::query_as(r#"SELECT * FROM "devices""#)
            .fetch_all(db)
            .await
    }

    pub async fn all_by_profile(db: &DbPool, profile_id: ProfileId) -> DbResult<Vec<DeviceModel>> {
        sqlx::query_as(r#"SELECT * FROM "devices" WHERE "profile_id" = ?"#)
            .bind(profile_id)
            .fetch_all(db)
            .await
    }

    pub async fn all_by_folder(db: &DbPool, folder_id: FolderId) -> DbResult<Vec<DeviceModel>> {
        sqlx::query_as(r#"SELECT * FROM "devices" WHERE "folder_id" = ?"#)
            .bind(folder_id)
            .fetch_all(db)
            .await
    }

    pub async fn delete(db: &DbPool, device_id: DeviceId) -> DbResult<()> {
        sqlx::query(r#"DELETE FROM "devices" WHERE "id" = ?"#)
            .bind(device_id)
            .execute(db)
            .await?;
        Ok(())
    }
}
