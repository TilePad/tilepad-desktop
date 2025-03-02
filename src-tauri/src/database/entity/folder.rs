use sea_query::{Expr, IdenStatic, Order, Query};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::database::{
    helpers::{sql_exec, sql_query_all, sql_query_maybe_one, UpdateStatementExt},
    DbPool, DbResult,
};

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

#[derive(Deserialize)]
pub struct UpdateFolder {
    pub name: Option<String>,
    pub config: Option<FolderConfig>,
    pub profile_id: Option<ProfileId>,
    pub default: Option<bool>,
    pub order: Option<u32>,
}

impl FolderModel {
    /// Create a new profile
    pub async fn create(db: &DbPool, create: CreateFolder) -> anyhow::Result<FolderModel> {
        let model = FolderModel {
            id: Uuid::new_v4(),
            name: create.name,
            default: create.default,
            profile_id: create.profile_id,
            config: create.config,
            order: create.order,
        };

        let config = serde_json::to_value(&model.config)?;

        sql_exec(
            db,
            Query::insert()
                .into_table(FoldersTable)
                .columns([
                    FoldersColumn::Id,
                    FoldersColumn::Name,
                    FoldersColumn::Default,
                    FoldersColumn::ProfileId,
                    FoldersColumn::Config,
                    FoldersColumn::Order,
                ])
                .values_panic([
                    model.id.into(),
                    model.name.clone().into(),
                    model.default.into(),
                    model.profile_id.into(),
                    config.into(),
                    model.order.into(),
                ]),
        )
        .await?;

        Ok(model)
    }

    pub async fn get_by_id(db: &DbPool, folder_id: FolderId) -> DbResult<Option<FolderModel>> {
        sql_query_maybe_one(
            db,
            Query::select()
                .from(FoldersTable)
                .columns([
                    FoldersColumn::Id,
                    FoldersColumn::Name,
                    FoldersColumn::Default,
                    FoldersColumn::ProfileId,
                    FoldersColumn::Config,
                    FoldersColumn::Order,
                ])
                .and_where(Expr::col(FoldersColumn::Id).eq(folder_id)),
        )
        .await
    }

    pub async fn update(
        mut self,
        db: &DbPool,
        update: UpdateFolder,
    ) -> anyhow::Result<FolderModel> {
        sql_exec(
            db,
            Query::update()
                .table(FoldersTable)
                .and_where(Expr::col(FoldersColumn::Id).eq(self.id))
                .cond_value(FoldersColumn::Name, update.name.as_ref())
                .cond_value_json(FoldersColumn::Config, update.config.as_ref())?
                .cond_value(FoldersColumn::ProfileId, update.profile_id)
                .cond_value(FoldersColumn::Order, update.order),
        )
        .await?;

        self.name = update.name.unwrap_or(self.name);
        self.config = update.config.unwrap_or(self.config);
        self.profile_id = update.profile_id.unwrap_or(self.profile_id);
        self.order = update.order.unwrap_or(self.order);

        if let Some(true) = update.default {
            self.set_default(db).await?;
        }

        Ok(self)
    }

    /// Set this profile as the default profile
    pub async fn set_default(&mut self, db: &DbPool) -> DbResult<()> {
        sql_exec(
            db,
            Query::update().table(FoldersTable).value(
                FoldersColumn::Default,
                Expr::case(Expr::col(FoldersColumn::Id).eq(self.id), true).finally(false),
            ),
        )
        .await?;

        self.default = true;

        Ok(())
    }

    pub async fn all(db: &DbPool, profile_id: ProfileId) -> DbResult<Vec<FolderModel>> {
        sql_query_all(
            db,
            Query::select()
                .from(FoldersTable)
                .columns([
                    FoldersColumn::Id,
                    FoldersColumn::Name,
                    FoldersColumn::Default,
                    FoldersColumn::ProfileId,
                    FoldersColumn::Config,
                    FoldersColumn::Order,
                ])
                .and_where(Expr::col(FoldersColumn::ProfileId).eq(profile_id))
                .order_by(FoldersColumn::Order, Order::Asc),
        )
        .await
    }

    pub async fn delete(db: &DbPool, folder_id: FolderId) -> DbResult<()> {
        sql_exec(
            db,
            Query::delete()
                .from_table(FoldersTable)
                .and_where(Expr::col(FoldersColumn::Id).eq(folder_id))
                .and_where(Expr::col(FoldersColumn::Default).eq(false)),
        )
        .await
    }

    /// Get the first profile marked as default
    pub async fn get_default(db: &DbPool, profile_id: ProfileId) -> DbResult<Option<FolderModel>> {
        sql_query_maybe_one(
            db,
            Query::select()
                .from(FoldersTable)
                .columns([
                    FoldersColumn::Id,
                    FoldersColumn::Name,
                    FoldersColumn::Default,
                    FoldersColumn::ProfileId,
                    FoldersColumn::Config,
                    FoldersColumn::Order,
                ])
                .and_where(Expr::col(FoldersColumn::ProfileId).eq(profile_id))
                .and_where(Expr::col(FoldersColumn::Default).eq(true)),
        )
        .await
    }
}

#[derive(IdenStatic, Copy, Clone)]
#[iden(rename = "folders")]
pub struct FoldersTable;

#[derive(IdenStatic, Copy, Clone)]
pub enum FoldersColumn {
    /// Unique ID for the folder
    Id,
    /// Name of the folder
    Name,
    /// Folder config (JSON)
    Config,
    /// Order in the list for this folder
    Order,
    /// Profile the folder belongs to
    ProfileId,
    /// Whether the folder is the default for the profile
    Default,
}
