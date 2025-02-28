use sea_query::{Expr, IdenStatic, Order, Query};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::database::{
    helpers::{sql_exec, sql_query_all, sql_query_maybe_one},
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
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FolderConfig {}

pub struct CreateFolder {
    pub name: String,
    pub config: FolderConfig,
    pub profile_id: ProfileId,
    pub default: bool,
    pub order: u32,
}

pub struct UpdateFolder {}

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

    /// Get the default folder for a profile
    pub async fn get_default_folder(
        db: &DbPool,
        profile_id: ProfileId,
    ) -> DbResult<Option<FolderModel>> {
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
                .and_where(Expr::col(FoldersColumn::Default).eq(true))
                .and_where(Expr::col(FoldersColumn::ProfileId).eq(profile_id)),
        )
        .await
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
