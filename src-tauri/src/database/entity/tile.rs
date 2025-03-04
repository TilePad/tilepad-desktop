use crate::{
    database::{
        helpers::{sql_exec, sql_query_all, sql_query_maybe_one, UpdateStatementExt},
        DbPool, DbResult,
    },
    plugin::manifest::{ActionId, PluginId},
};

use super::folder::FolderId;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileConfig {
    /// ID of the plugin the action we are executing is withins
    pub plugin_id: PluginId,
    /// ID of the action to execution
    pub action_id: ActionId,
    /// Icon to use
    #[serde(default)]
    pub icon: TileIcon,
    /// Configuration for the action
    pub properties: serde_json::Value,

    #[serde(default)]
    pub label: TileLabel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TileLabel {
    pub enabled: bool,
    pub label: String,
    pub align: LabelAlign,

    pub font_size: u32,

    pub bold: bool,
    pub italic: bool,
    pub underline: bool,

    pub color: String,
}

impl Default for TileLabel {
    fn default() -> Self {
        Self {
            enabled: true,
            label: Default::default(),
            align: Default::default(),
            font_size: 10,
            bold: false,
            italic: false,
            underline: false,
            color: "#ffffff".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LabelAlign {
    #[default]
    Bottom,
    Middle,
    Top,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TileIcon {
    #[default]
    None,
    PluginIcon {
        plugin_id: PluginId,
        icon: String,
    },
}

#[derive(Deserialize)]
pub struct CreateTile {
    pub config: TileConfig,
    pub folder_id: FolderId,
    pub row: u32,
    pub column: u32,
}

#[derive(Deserialize)]
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

    pub async fn update(mut self, db: &DbPool, update: UpdateTile) -> anyhow::Result<TileModel> {
        sql_exec(
            db,
            Query::update()
                .table(TilesTable)
                .and_where(Expr::col(TilesColumn::Id).eq(self.id))
                .cond_value_json(TilesColumn::Config, update.config.as_ref())?
                .cond_value(TilesColumn::FolderId, update.folder_id)
                .cond_value(TilesColumn::Column, update.column)
                .cond_value(TilesColumn::Row, update.row),
        )
        .await?;

        self.config = update.config.unwrap_or(self.config);
        self.folder_id = update.folder_id.unwrap_or(self.folder_id);
        self.column = update.column.unwrap_or(self.column);
        self.row = update.row.unwrap_or(self.row);

        Ok(self)
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

    pub async fn get_by_id(db: &DbPool, tile_id: TileId) -> DbResult<Option<TileModel>> {
        sql_query_maybe_one(
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
                .and_where(Expr::col(TilesColumn::Id).eq(tile_id)),
        )
        .await
    }

    pub async fn delete(db: &DbPool, tile_id: TileId) -> DbResult<()> {
        sql_exec(
            db,
            Query::delete()
                .from_table(TilesTable)
                .and_where(Expr::col(TilesColumn::Id).eq(tile_id)),
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
