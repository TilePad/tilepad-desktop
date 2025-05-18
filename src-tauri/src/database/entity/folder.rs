use crate::database::{DbErr, DbPool, DbResult};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use super::profile::ProfileId;

pub type FolderId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FolderModel {
    pub id: FolderId,
    pub name: String,
    #[sqlx(json)]
    pub config: FolderConfig,
    pub profile_id: ProfileId,
    pub default: bool,
    pub order: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FolderConfig {
    pub rows: u32,
    pub columns: u32,
}

impl Default for FolderConfig {
    fn default() -> Self {
        Self {
            rows: 4,
            columns: 6,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateFolder {
    pub name: String,
    pub config: FolderConfig,
    pub profile_id: ProfileId,
    pub default: bool,
    pub order: u32,
}

impl FolderModel {
    /// Create a new profile
    pub async fn create(db: &DbPool, create: CreateFolder) -> DbResult<FolderModel> {
        let model = FolderModel {
            id: Uuid::new_v4(),
            name: create.name,
            default: create.default,
            profile_id: create.profile_id,
            config: create.config,
            order: create.order,
        };

        let config =
            serde_json::to_value(&model.config).map_err(|err| DbErr::Encode(err.into()))?;
        sqlx::query(
            r#"
            INSERT INTO "folders" ("id", "name", "default", "profile_id", "config", "order")
            VALUES (?, ?, ?, ?, ?, ?)
        "#,
        )
        .bind(model.id)
        .bind(model.name.clone())
        .bind(model.default)
        .bind(model.profile_id)
        .bind(config)
        .bind(model.order)
        .execute(db)
        .await?;

        Ok(model)
    }

    pub async fn get_by_id(db: &DbPool, folder_id: FolderId) -> DbResult<Option<FolderModel>> {
        sqlx::query_as(r#"SELECT * FROM "folders" WHERE "id" = ?"#)
            .bind(folder_id)
            .fetch_optional(db)
            .await
    }

    pub async fn set_name(mut self, db: &DbPool, name: String) -> DbResult<FolderModel> {
        sqlx::query(r#"UPDATE "folders" SET "name" = ? WHERE "id" = ?"#)
            .bind(&name)
            .bind(self.id)
            .execute(db)
            .await?;

        self.name = name;

        Ok(self)
    }

    pub async fn set_config(mut self, db: &DbPool, config: FolderConfig) -> DbResult<FolderModel> {
        let config_json = serde_json::to_value(&config).map_err(|err| DbErr::Encode(err.into()))?;

        sqlx::query(r#"UPDATE "folders" SET "config" = ? WHERE "id" = ?"#)
            .bind(config_json)
            .bind(self.id)
            .execute(db)
            .await?;

        self.config = config;
        Ok(self)
    }

    /// Set this profile as the default profile
    #[allow(unused)]
    pub async fn set_default(&mut self, db: &DbPool) -> DbResult<()> {
        sqlx::query(
            r#"UPDATE "folders" SET "default" = CASE WHEN "id" = ? THEN TRUE ELSE FALSE END"#,
        )
        .bind(self.id)
        .execute(db)
        .await?;

        self.default = true;

        Ok(())
    }

    pub async fn all(db: &DbPool, profile_id: ProfileId) -> DbResult<Vec<FolderModel>> {
        sqlx::query_as(r#"SELECT * FROM "folders" WHERE "profile_id" = ? ORDER BY "order" ASC"#)
            .bind(profile_id)
            .fetch_all(db)
            .await
    }

    pub async fn delete(db: &DbPool, folder_id: FolderId) -> DbResult<()> {
        sqlx::query(r#"DELETE FROM "folders" WHERE "id" = ? AND "default" = FALSE"#)
            .bind(folder_id)
            .execute(db)
            .await?;
        Ok(())
    }

    /// Get the first profile marked as default
    pub async fn get_default(db: &DbPool, profile_id: ProfileId) -> DbResult<Option<FolderModel>> {
        sqlx::query_as(r#"SELECT * FROM "folders" WHERE "profile_id" = ? AND "default" = TRUE"#)
            .bind(profile_id)
            .fetch_optional(db)
            .await
    }
}
