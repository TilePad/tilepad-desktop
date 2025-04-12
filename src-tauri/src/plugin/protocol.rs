use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::{
        JsonObject,
        entity::{
            folder::FolderModel,
            tile::{TileId, TileModel},
        },
    },
    events::{DeepLinkContext, InspectorContext, TileInteractionContext},
};

use super::manifest::{ActionId, PluginId};

/// Plugin message coming from the client side
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientPluginMessage {
    /// Register the current plugin with the server
    RegisterPlugin { plugin_id: PluginId },

    /// Request the current plugin properties
    GetProperties,

    /// Set the properties for the plugin (Partial update)
    SetProperties {
        properties: JsonObject,

        /// Whether to treat the properties update as a partial update
        #[serde(default = "default_partial_value")]
        partial: bool,
    },

    /// Send data to the current inspector window
    SendToInspector {
        /// Inspector context
        ctx: InspectorContext,
        /// Message to send the inspector
        message: serde_json::Value,
    },

    /// Open a URL
    OpenUrl { url: String },

    /// Request the current properties for a tile
    GetTileProperties {
        /// ID of the tile to get properties for
        tile_id: TileId,
    },

    /// Set the current properties for a tile
    SetTileProperties {
        /// ID of the tile to set properties for
        tile_id: TileId,
        /// Properties for the tile
        properties: JsonObject,
        /// Whether to treat the properties update as a partial update
        #[serde(default = "default_partial_value")]
        partial: bool,
    },
}

fn default_partial_value() -> bool {
    true
}

/// Plugin message coming from the server side
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ServerPluginMessage {
    /// Plugin has registered with the server
    Registered { plugin_id: PluginId },

    /// Properties received from the server
    Properties { properties: JsonObject },

    /// Tile was clicked on a remote device
    TileClicked {
        ctx: TileInteractionContext,
        properties: JsonObject,
    },

    /// Got a message from the inspector
    RecvFromInspector {
        ctx: InspectorContext,
        message: serde_json::Value,
    },

    /// Inspector was opened
    InspectorOpen { ctx: InspectorContext },

    /// Inspector was closed
    InspectorClose { ctx: InspectorContext },

    /// Received a deep link message for the plugin
    DeepLink { ctx: DeepLinkContext },

    /// Properties requested for a tile
    TileProperties {
        tile_id: TileId,
        properties: JsonObject,
    },
}
