use crate::database::{
    helpers::{sql_exec, sql_query_all},
    DbPool, DbResult,
};

use super::folder::FolderId;
use chrono::{DateTime, Utc};
use sea_query::{Expr, IdenStatic, Query};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

pub type TileId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TileModel {
    pub id: TileId,
    #[sqlx(json)]
    pub config: TileConfig,
    pub folder_id: FolderId,
    pub row: u32,
    pub column: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TileConfig {
    /// ID of the action to execution
    pub action_id: String,
    /// Configuration for the action
    pub action_config: serde_json::Value,
}

pub struct CreateTile {
    pub config: TileConfig,
    pub folder_id: FolderId,
    pub row: u32,
    pub column: u32,
}

pub struct UpdateTile {
    pub config: Option<TileConfig>,
    pub folder_id: Option<FolderId>,
    pub row: Option<u32>,
    pub column: Option<u32>,
}

impl TileModel {
    /// Create a new profile
    pub async fn create(db: &DbPool, create: CreateTile) -> anyhow::Result<TileModel> {
        let model = TileModel {
            id: Uuid::new_v4(),
            config: create.config,
            folder_id: create.folder_id,
            row: create.row,
            column: create.column,
        };

        let config = serde_json::to_value(&model.config)?;

        sql_exec(
            db,
            Query::insert()
                .into_table(TilesTable)
                .columns([
                    TilesColumn::Id,
                    TilesColumn::Config,
                    TilesColumn::FolderId,
                    TilesColumn::Row,
                    TilesColumn::Column,
                ])
                .values_panic([
                    model.id.into(),
                    config.into(),
                    model.folder_id.into(),
                    model.row.into(),
                    model.column.into(),
                ]),
        )
        .await?;

        Ok(model)
    }

    pub async fn update(&mut self, db: &DbPool, update: UpdateTile) -> anyhow::Result<()> {
        let mut query = Query::update();
        query
            .table(TilesTable)
            .and_where(Expr::col(TilesColumn::Id).eq(self.id));

        if let Some(config) = update.config {
            let value = serde_json::to_value(&config)?;
            self.config = config;
            query.value(TilesColumn::Config, value);
        }

        if let Some(folder_id) = update.folder_id {
            self.folder_id = folder_id;
            query.value(TilesColumn::FolderId, folder_id);
        }

        if let Some(column) = update.column {
            self.column = column;
            query.value(TilesColumn::Column, column);
        }

        if let Some(row) = update.row {
            self.row = row;
            query.value(TilesColumn::Row, row);
        }

        sql_exec(db, &query).await?;
        Ok(())
    }

    pub async fn get_by_folder(db: &DbPool, folder_id: FolderId) -> DbResult<Vec<TileModel>> {
        sql_query_all(
            db,
            Query::select()
                .from(TilesTable)
                .columns([
                    TilesColumn::Id,
                    TilesColumn::Config,
                    TilesColumn::FolderId,
                    TilesColumn::Row,
                    TilesColumn::Column,
                ])
                .and_where(Expr::col(TilesColumn::FolderId).eq(folder_id)),
        )
        .await
    }
}

#[derive(IdenStatic, Copy, Clone)]
#[iden(rename = "tiles")]
pub struct TilesTable;

#[derive(IdenStatic, Copy, Clone)]
pub enum TilesColumn {
    /// Unique ID for the tile
    Id,
    /// Tile configuration (JSON)
    Config,
    /// ID of a folder this tile is within
    FolderId,
    /// Row the tile is on
    Row,
    /// Column the tile is on
    Column,
}
