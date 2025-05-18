use crate::database::{DbErr, DbPool, DbResult};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

pub type ProfileId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProfileModel {
    pub id: ProfileId,
    pub name: String,
    pub default: bool,
    #[sqlx(json)]
    pub config: ProfileConfig,
    pub order: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {}

#[derive(Deserialize)]
pub struct CreateProfile {
    pub name: String,
    pub default: bool,
    pub config: ProfileConfig,
    pub order: u32,
}

impl ProfileModel {
    /// Create a new profile
    pub async fn create(db: &DbPool, create: CreateProfile) -> DbResult<ProfileModel> {
        let mut model = ProfileModel {
            id: Uuid::new_v4(),
            name: create.name,
            default: false,
            config: create.config,
            order: create.order,
        };

        let config =
            serde_json::to_value(&model.config).map_err(|err| DbErr::Encode(err.into()))?;

        sqlx::query(
            r#"
            INSERT INTO "profiles" ("id", "name", "default", "config", "order")
            VALUES (?, ?, ?, ?, ?)
        "#,
        )
        .bind(model.id)
        .bind(model.name.clone())
        .bind(model.default)
        .bind(config)
        .bind(model.order)
        .execute(db)
        .await?;

        if create.default {
            model = model.set_default(db).await?;
        }

        Ok(model)
    }

    pub async fn get_by_id(db: &DbPool, id: ProfileId) -> DbResult<Option<ProfileModel>> {
        sqlx::query_as(r#"SELECT * FROM "profiles" WHERE "id" = ?"#)
            .bind(id)
            .fetch_optional(db)
            .await
    }

    /// Update the name of the profile
    pub async fn set_name(mut self, db: &DbPool, name: String) -> DbResult<ProfileModel> {
        sqlx::query(r#"UPDATE "profiles" SET "name" = ? WHERE "id" = ?"#)
            .bind(&name)
            .bind(self.id)
            .execute(db)
            .await?;

        self.name = name;

        Ok(self)
    }

    /// Set this profile as the default profile
    pub async fn set_default(mut self, db: &DbPool) -> DbResult<ProfileModel> {
        sqlx::query(
            r#"UPDATE "profiles" SET "default" = CASE WHEN "id" = ? THEN TRUE ELSE FALSE END"#,
        )
        .bind(self.id)
        .execute(db)
        .await?;

        self.default = true;

        Ok(self)
    }

    /// Get the first profile marked as default
    pub async fn get_default_profile(db: &DbPool) -> DbResult<Option<ProfileModel>> {
        sqlx::query_as(r#"SELECT * FROM "profiles" WHERE "default" = TRUE"#)
            .fetch_optional(db)
            .await
    }

    pub async fn all(db: &DbPool) -> DbResult<Vec<ProfileModel>> {
        sqlx::query_as(r#"SELECT * FROM "profiles" ORDER BY "order" ASC"#)
            .fetch_all(db)
            .await
    }

    pub async fn delete(db: &DbPool, profile_id: ProfileId) -> DbResult<()> {
        sqlx::query(r#"DELETE FROM "profiles" WHERE "id" = ? AND "default" = FALSE"#)
            .bind(profile_id)
            .execute(db)
            .await?;
        Ok(())
    }
}
