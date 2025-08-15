use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::{
        JsonObject,
        entity::{
            device::DeviceId,
            tile::{TileIcon, TileId, TileLabel, TileModel},
        },
    },
    device::protocol::DeviceIndicator,
    events::{DeepLinkContext, DisplayContext, InspectorContext, TileInteractionContext},
};

use tilepad_manifest::plugin::PluginId;

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

    /// Send data to a specific display
    SendToDisplay {
        /// Inspector context
        ctx: DisplayContext,
        /// Message to send the display
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

    /// Set the current icon for a tile
    SetTileIcon { tile_id: TileId, icon: TileIcon },

    /// Set the current label for a tile
    SetTileLabel { tile_id: TileId, label: TileLabel },

    /// Get all currently visible tiles
    GetVisibleTiles,

    /// Display an icon on connected devices
    DisplayIndicator {
        /// ID of the device to display on
        device_id: Uuid,
        /// ID of the tile to display it on
        tile_id: Uuid,
        /// Indicator to display
        indicator: DeviceIndicator,
        /// Duration in milliseconds to display the
        /// indicator for
        duration: u32,
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

    /// Got a message from a display
    RecvFromDisplay {
        ctx: DisplayContext,
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

    /// Selection of tiles for a device has changed
    DeviceTiles {
        /// ID of the device that changes
        device_id: DeviceId,
        /// Tiles that are now visible on the device
        tiles: Vec<TileModel>,
    },

    VisibleTiles {
        /// Tiles that are currently visible
        tiles: Vec<TileModel>,
    },
}
