use crate::{
    database::{DbErr, DbPool, DbResult, JsonObject},
    plugin::manifest::{ActionId, PluginId},
};

use super::folder::FolderId;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tilepad_manifest::icons::IconPackId;
use uuid::Uuid;

pub type TileId = Uuid;

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
    pub async fn create(db: &DbPool, create: CreateTile) -> DbResult<TileModel> {
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

        let config =
            serde_json::to_value(&model.config).map_err(|err| DbErr::Encode(err.into()))?;
        let properties = serde_json::Value::Object(Default::default());

        sqlx::query(
            "
            INSERT INTO \"tiles\" (\"id\", \"config\", \"properties\", \"folder_id\", \"plugin_id\", \"action_id\", \"row\", \"column\")
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        ",
        )
        .bind(model.id)
        .bind(config)
        .bind(properties)
        .bind(model.folder_id)
        .bind(model.plugin_id.0.as_str())
        .bind(model.action_id.0.as_str())
        .bind(model.row)
        .bind(model.column)
        .execute(db)
        .await?;

        Ok(model)
    }

    /// Update the properties for the tile
    pub async fn update_properties(
        mut self,
        db: &DbPool,
        properties: JsonObject,
        partial: bool,
    ) -> DbResult<TileModel> {
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

        sqlx::query("UPDATE \"tiles\" SET \"properties\" = ? WHERE \"id\" = ?")
            .bind(serde_json::Value::Object(properties.clone()))
            .bind(self.id)
            .execute(db)
            .await?;

        self.properties = properties;
        Ok(self)
    }

    pub async fn update_config(mut self, db: &DbPool, config: TileConfig) -> DbResult<TileModel> {
        let config_json = serde_json::to_value(&config).map_err(|err| DbErr::Encode(err.into()))?;

        sqlx::query("UPDATE \"tiles\" SET \"config\" = ? WHERE \"id\" = ?")
            .bind(config_json)
            .bind(self.id)
            .execute(db)
            .await?;

        self.config = config;
        Ok(self)
    }

    pub async fn update_position(
        mut self,
        db: &DbPool,
        row: u32,
        column: u32,
    ) -> DbResult<TileModel> {
        sqlx::query("UPDATE \"tiles\" SET \"row\" = ?, \"column\" = ? WHERE \"id\" = ?")
            .bind(row)
            .bind(column)
            .bind(self.id)
            .execute(db)
            .await?;

        self.row = row;
        self.column = column;
        Ok(self)
    }

    /// Update the label portion of the config
    pub async fn update_label(
        self,
        db: &DbPool,
        label: TileLabel,
        kind: UpdateKind,
    ) -> DbResult<TileModel> {
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

        self.update_config(db, new_config).await
    }

    /// Update the icon portion of the config
    pub async fn update_icon(
        self,
        db: &DbPool,
        icon: TileIcon,
        kind: UpdateKind,
    ) -> DbResult<TileModel> {
        // Icon update is ignored if the user has already set a icon and
        // the update is from plugin / inspector
        if matches!(kind, UpdateKind::Program) && self.config.user_flags.icon {
            return Ok(self);
        }

        let mut new_config = self.config.clone();
        new_config.icon = icon;
        new_config.user_flags.icon = matches!(kind, UpdateKind::User);

        self.update_config(db, new_config).await
    }

    /// Update the icon portion of the config
    pub async fn update_icon_options(
        self,
        db: &DbPool,
        icon_options: TileIconOptions,
    ) -> DbResult<TileModel> {
        let mut new_config = self.config.clone();
        new_config.icon_options = icon_options;

        self.update_config(db, new_config).await
    }

    pub async fn get_by_folder(db: &DbPool, folder_id: FolderId) -> DbResult<Vec<TileModel>> {
        sqlx::query_as("SELECT * FROM \"tiles\" WHERE \"folder_id\" = ?")
            .bind(folder_id)
            .fetch_all(db)
            .await
    }

    pub async fn get_by_id(db: &DbPool, tile_id: TileId) -> DbResult<Option<TileModel>> {
        sqlx::query_as("SELECT * FROM \"tiles\" WHERE \"id\" = ?")
            .bind(tile_id)
            .fetch_optional(db)
            .await
    }

    pub async fn delete(db: &DbPool, tile_id: TileId) -> DbResult<()> {
        sqlx::query("DELETE FROM \"tiles\" WHERE \"id\" = ?")
            .bind(tile_id)
            .execute(db)
            .await?;
        Ok(())
    }
}
