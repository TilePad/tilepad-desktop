use crate::{
    database::entity::{device::DeviceId, folder::FolderId, profile::ProfileId, tile::TileId},
    device::DeviceRequestId,
    plugin::runner::PluginTaskState,
};
use serde::{Deserialize, Serialize};
use tilepad_manifest::icons::IconPackId;
use tilepad_manifest::plugin::{ActionId, PluginId};
use tokio::sync::mpsc;

pub mod processing;

pub type AppEventSender = mpsc::UnboundedSender<AppEvent>;
pub type AppEventReceiver = mpsc::UnboundedReceiver<AppEvent>;

#[derive(Debug)]
pub enum AppEvent {
    DeviceRequest(DeviceRequestAppEvent),

    Device(DeviceAppEvent),

    Plugin(PluginAppEvent),

    IconPack(IconPackAppEvent),
}

#[derive(Debug)]
pub enum DeviceAppEvent {
    Authenticated { device_id: DeviceId },
    Revoked { device_id: DeviceId },
    Disconnected { device_id: DeviceId },
}

#[derive(Debug)]
pub enum DeviceRequestAppEvent {
    Added { request_id: DeviceRequestId },
    Removed { request_id: DeviceRequestId },

    Accepted { request_id: DeviceRequestId },
    Decline { request_id: DeviceRequestId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectorContext {
    pub profile_id: ProfileId,
    pub folder_id: FolderId,

    pub plugin_id: PluginId,
    pub action_id: ActionId,

    pub tile_id: TileId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayContext {
    pub plugin_id: PluginId,
    pub action_id: ActionId,
    pub tile_id: TileId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileInteractionContext {
    pub device_id: DeviceId,

    pub plugin_id: PluginId,
    pub action_id: ActionId,

    pub tile_id: TileId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepLinkContext {
    pub url: String,
    pub host: Option<String>,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

#[derive(Debug)]
pub enum PluginAppEvent {
    /// Got a message from the plugin for the inspector
    Message {
        context: InspectorContext,
        message: serde_json::Value,
    },
    /// Got a message from the plugin for a display
    DisplayMessage {
        context: DisplayContext,
        message: serde_json::Value,
    },

    /// Plugin was loaded
    Loaded { plugin_id: PluginId },

    /// Plugin was unloaded
    Unloaded { plugin_id: PluginId },

    /// Plugin task state has changed
    TaskStateChanged {
        plugin_id: PluginId,
        state: PluginTaskState,
    },
}

#[derive(Debug)]
pub enum IconPackAppEvent {
    Loaded { pack_id: IconPackId },

    Unloaded { pack_id: IconPackId },
}
