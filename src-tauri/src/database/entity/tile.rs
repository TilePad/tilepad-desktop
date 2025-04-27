use crate::{
    database::{
        DbPool, DbResult, JsonObject,
        helpers::{UpdateStatementExt, sql_exec, sql_query_all, sql_query_maybe_one},
    },
    plugin::manifest::{ActionId, PluginId},
};

use super::folder::FolderId;
use sea_query::{Expr, IdenStatic, Query};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tilepad_manifest::icons::IconPackId;
use uuid::Uuid;

pub type TileId = Uuid;

#[derive(IdenStatic, Copy, Clone)]
#[iden(rename = "tiles")]
pub struct TilesTable;

#[derive(IdenStatic, Copy, Clone)]
pub enum TilesColumn {
    /// Unique ID for the tile
    Id,
    /// Tile configuration (JSON)
    Config,
    /// Plugin properties for this tile
    Properties,
    /// ID of a folder this tile is within
    FolderId,
    /// ID of the plugin the action is apart of
    PluginId,
    /// ID of the action within the plugin to execute
    ActionId,
    /// Row the tile is on
    Row,
    /// Column the tile is on
    Column,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TileModel {
    /// Unique ID of the tile
    pub id: TileId,

    /// Configuration for the tile and how it appears in the UI
    #[sqlx(json)]
    pub config: TileConfig,

    /// Properties / settings defined on this specific tile
    #[sqlx(json)]
    pub properties: JsonObject,

    /// ID of the folder this tile is within
    pub folder_id: FolderId,

    /// ID of the plugin the `action_id` is apart of
    #[sqlx(try_from = "String")]
    pub plugin_id: PluginId,
    /// ID of the action within the plugin to execute
    #[sqlx(try_from = "String")]
    pub action_id: ActionId,

    /// Row within the UI to display at
    pub row: u32,
    /// Column within the UI to display at
    pub column: u32,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TileConfig {
    /// Icon to use
    pub icon: TileIcon,
    /// Icon options to use
    pub icon_options: TileIconOptions,
    /// Label to display on top of the tile
    pub label: TileLabel,
    /// States for whether a part of the config has been modified
    /// by the user or not
    pub user_flags: UserFlags,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UserFlags {
    /// User has modified the icon
    pub icon: bool,

    /// User has modified the label
    pub label: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TileIconOptions {
    pub padding: u32,
    pub background_color: String,
}

impl Default for TileIconOptions {
    fn default() -> Self {
        Self {
            padding: 0,
            background_color: "#00000000".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TileLabel {
    pub enabled: bool,
    pub label: String,
    pub align: LabelAlign,

    pub font: String,
    pub font_size: u32,

    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub outline: bool,

    pub color: String,
    pub outline_color: String,
}

impl Default for TileLabel {
    fn default() -> Self {
        Self {
            enabled: true,
            label: Default::default(),
            align: Default::default(),
            font: "Roboto".to_string(),
            font_size: 10,
            bold: false,
            italic: false,
            underline: false,
            outline: true,
            color: "#ffffff".to_string(),
            outline_color: "#000000".to_string(),
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

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum TileIcon {
    /// No icon
    #[default]
    None,

    /// Icon from a specific plugin path
    PluginIcon {
        /// ID of the plugin the icon is from
        plugin_id: PluginId,
        /// Path to the icon file
        icon: String,
    },

    /// Use an icon from an icon pack
    IconPack {
        /// ID of the icon pack
        pack_id: IconPackId,
        /// Path to the icon file
        path: String,
    },

    // Image at some remote URL
    Url {
        src: String,
    },

    /// User uploaded file
    Uploaded {
        /// Path to the uploaded file
        path: String,
    },
}

#[derive(Deserialize)]
pub struct CreateTile {
    pub config: TileConfig,
    pub folder_id: FolderId,

    pub plugin_id: PluginId,
    pub action_id: ActionId,

    pub row: u32,
    pub column: u32,
}

#[derive(Deserialize)]
pub enum UpdateKind {
    /// Resetting to default value
    Reset,

    /// User requested the change
    User,

    /// Change was made by the plugin or an inspector
    Program,
}

impl TileModel {
    /// Create a new profile
    pub async fn create(db: &DbPool, create: CreateTile) -> anyhow::Result<TileModel> {
        let model = TileModel {
            id: Uuid::new_v4(),
            config: create.config,
            properties: Default::default(),
            folder_id: create.folder_id,
            plugin_id: create.plugin_id,
            action_id: create.action_id,
            row: create.row,
            column: create.column,
        };

        let config = serde_json::to_value(&model.config)?;
        let properties = serde_json::Value::Object(Default::default());

        sql_exec(
            db,
            Query::insert()
                .into_table(TilesTable)
                .columns([
                    TilesColumn::Id,
                    TilesColumn::Config,
                    TilesColumn::Properties,
                    TilesColumn::FolderId,
                    TilesColumn::PluginId,
                    TilesColumn::ActionId,
                    TilesColumn::Row,
                    TilesColumn::Column,
                ])
                .values_panic([
                    model.id.into(),
                    config.into(),
                    properties.into(),
                    model.folder_id.into(),
                    model.plugin_id.0.clone().into(),
                    model.action_id.0.clone().into(),
                    model.row.into(),
                    model.column.into(),
                ]),
        )
        .await?;

        Ok(model)
    }

    /// Update the properties for the tile
    pub async fn update_properties(
        mut self,
        db: &DbPool,
        properties: JsonObject,
        partial: bool,
    ) -> anyhow::Result<TileModel> {
        let properties = if partial {
            let mut existing_properties = self.properties.clone();
            // Merge the new properties onto the old
            for (key, value) in properties {
                existing_properties.insert(key, value);
            }
            existing_properties
        } else {
            properties
        };

        sql_exec(
            db,
            Query::update()
                .table(TilesTable)
                .and_where(Expr::col(TilesColumn::Id).eq(self.id))
                .value_json(TilesColumn::Properties, &properties)?,
        )
        .await?;

        self.properties = properties;
        Ok(self)
    }

    /// Update the label portion of the config
    pub async fn update_label(
        mut self,
        db: &DbPool,
        label: TileLabel,
        kind: UpdateKind,
    ) -> anyhow::Result<TileModel> {
        // Label update is ignored if the user has already set a label and
        // the update is from plugin / inspector
        if matches!(kind, UpdateKind::Program) && self.config.user_flags.label {
            return Ok(self);
        }

        let mut new_config = self.config.clone();
        new_config.label = label;

        // User label is only dirty if its not empty
        new_config.user_flags.label = match kind {
            // Label is now considered untouched
            UpdateKind::Reset => false,

            // Only touched if the label is non empty
            UpdateKind::User => !new_config.label.label.is_empty(),

            // Not touched by user, was made by the plugin / inspector
            UpdateKind::Program => false,
        };

        sql_exec(
            db,
            Query::update()
                .table(TilesTable)
                .and_where(Expr::col(TilesColumn::Id).eq(self.id))
                .value_json(TilesColumn::Config, &new_config)?,
        )
        .await?;

        self.config = new_config;
        Ok(self)
    }

    /// Update the icon portion of the config
    pub async fn update_icon(
        mut self,
        db: &DbPool,
        icon: TileIcon,
        kind: UpdateKind,
    ) -> anyhow::Result<TileModel> {
        // Icon update is ignored if the user has already set a icon and
        // the update is from plugin / inspector
        if matches!(kind, UpdateKind::Program) && self.config.user_flags.icon {
            return Ok(self);
        }

        let mut new_config = self.config.clone();
        new_config.icon = icon;
        new_config.user_flags.icon = matches!(kind, UpdateKind::User);

        sql_exec(
            db,
            Query::update()
                .table(TilesTable)
                .and_where(Expr::col(TilesColumn::Id).eq(self.id))
                .value_json(TilesColumn::Config, &new_config)?,
        )
        .await?;

        self.config = new_config;
        Ok(self)
    }

    /// Update the icon portion of the config
    pub async fn update_icon_options(
        mut self,
        db: &DbPool,
        icon_options: TileIconOptions,
    ) -> anyhow::Result<TileModel> {
        let mut new_config = self.config.clone();
        new_config.icon_options = icon_options;

        sql_exec(
            db,
            Query::update()
                .table(TilesTable)
                .and_where(Expr::col(TilesColumn::Id).eq(self.id))
                .value_json(TilesColumn::Config, &new_config)?,
        )
        .await?;

        self.config = new_config;
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
                    TilesColumn::Properties,
                    TilesColumn::FolderId,
                    TilesColumn::PluginId,
                    TilesColumn::ActionId,
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
                    TilesColumn::Properties,
                    TilesColumn::FolderId,
                    TilesColumn::PluginId,
                    TilesColumn::ActionId,
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
