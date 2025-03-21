use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
    database::entity::{device::DeviceId, folder::FolderId, profile::ProfileId, tile::TileId},
    device::DeviceRequestId,
    plugin::manifest::{ActionId, PluginId},
};

pub mod processing;

pub type AppEventSender = mpsc::UnboundedSender<AppEvent>;
pub type AppEventReceiver = mpsc::UnboundedReceiver<AppEvent>;

#[derive(Debug)]
pub enum AppEvent {
    DeviceRequest(DeviceRequestAppEvent),

    Device(DeviceAppEvent),

    Plugin(PluginAppEvent),
}

#[derive(Debug)]
pub enum DeviceAppEvent {
    Authenticated { device_id: DeviceId },
    Revoked { device_id: DeviceId },
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
pub struct TileInteractionContext {
    pub device_id: DeviceId,

    pub plugin_id: PluginId,
    pub action_id: ActionId,

    pub tile_id: TileId,
}

#[derive(Debug)]
pub enum PluginAppEvent {
    RecvPluginMessage {
        context: InspectorContext,
        message: serde_json::Value,
    },

    OpenInspector {
        #[allow(unused)]
        context: InspectorContext,
    },

    CloseInspector {
        #[allow(unused)]
        context: InspectorContext,
    },
}
