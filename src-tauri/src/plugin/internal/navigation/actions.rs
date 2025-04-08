use std::sync::Arc;

use anyhow::Context;
use serde::Deserialize;
use tauri_plugin_opener::open_url;

use crate::{
    database::{
        entity::{
            device::{DeviceModel, UpdateDevice},
            folder::{FolderId, FolderModel},
            profile::{ProfileId, ProfileModel},
        },
        DbPool,
    },
    device::{protocol::ServerDeviceMessage, Devices},
    events::TileInteractionContext,
    plugin::Plugins,
};

#[derive(Deserialize)]
pub struct SwitchFolderProperties {
    folder: FolderId,
}

#[derive(Deserialize)]
pub struct SwitchProfileProperties {
    profile: ProfileId,
}

pub async fn handle(
    devices: &Arc<Devices>,
    plugins: &Arc<Plugins>,
    context: TileInteractionContext,
    properties: serde_json::Value,
) -> anyhow::Result<()> {
    match context.action_id.as_str() {
        "switch_folder" => {
            let data: SwitchFolderProperties = serde_json::from_value(properties)?;
            devices
                .update_device_folder(context.device_id, data.folder)
                .await?;
        }
        "switch_profile" => {
            let data: SwitchProfileProperties = serde_json::from_value(properties)?;
            devices
                .update_device_profile(context.device_id, data.profile)
                .await?;
        }
        action_id => {
            tracing::warn!(?action_id, ?context, "unknown internal action");
        }
    }
    Ok(())
}
