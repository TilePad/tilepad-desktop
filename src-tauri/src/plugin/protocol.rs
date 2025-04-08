use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::entity::{folder::FolderModel, tile::TileModel},
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
    SetProperties { properties: serde_json::Value },

    /// Send data to the current inspector window
    SendToInspector {
        /// Inspector context
        ctx: InspectorContext,
        /// Message to send the inspector
        message: serde_json::Value,
    },

    /// Open a URL
    OpenUrl { url: String },
}

/// Plugin message coming from the server side
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ServerPluginMessage {
    /// Plugin has registered with the server
    Registered { plugin_id: PluginId },

    /// Properties received from the server
    Properties { properties: serde_json::Value },

    /// Tile was clicked on a remote device
    TileClicked {
        ctx: TileInteractionContext,
        properties: serde_json::Value,
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
}
