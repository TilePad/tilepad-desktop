use anyhow::Context;
use serde::{de, Deserialize, Serialize};

use crate::{
    database::{
        entity::{
            device::{DeviceConfig, DeviceModel, UpdateDevice},
            folder::{FolderId, FolderModel},
            tile::TileModel,
        },
        DbPool,
    },
    device::{protocol::ServerDeviceMessage, Devices},
    events::{AppEventSender, DeviceMessageContext, PluginMessageContext},
    plugin::PluginRegistry,
};

pub async fn handle_internal_action(
    plugins: &PluginRegistry,
    devices: &Devices,
    db: &DbPool,
    context: DeviceMessageContext,
    tile: TileModel,
) -> anyhow::Result<()> {
    match context.plugin_id.as_str() {
        "com.tilepad.system.navigation" => {
            handle_internal_navigation(devices, plugins, db, context, tile).await?;
        }

        plugin_id => {
            tracing::warn!(?plugin_id, ?context, "unknown internal action");
        }
    }

    Ok(())
}

#[derive(Deserialize)]
pub struct SwitchFolderProperties {
    folder: FolderId,
}

async fn handle_internal_navigation(
    devices: &Devices,
    plugins: &PluginRegistry,
    db: &DbPool,
    context: DeviceMessageContext,
    tile: TileModel,
) -> anyhow::Result<()> {
    match context.action_id.as_str() {
        "switch_folder" => {
            let device = DeviceModel::get_by_id(db, context.device_id)
                .await?
                .context("device not found")?;
            let data: SwitchFolderProperties = serde_json::from_value(tile.config.properties)?;

            let folder_id = data.folder;
            let folder = FolderModel::get_by_id(db, folder_id)
                .await?
                .context("unknown folder")?;
            let tiles = TileModel::get_by_folder(db, folder_id).await?;

            let mut config = device.config.clone();
            config.folder_id = folder_id;

            device
                .update(
                    db,
                    UpdateDevice {
                        config: Some(config),
                        ..Default::default()
                    },
                )
                .await?;

            if let Some(session) = devices.get_session_by_device(context.device_id) {
                session.send_message(ServerDeviceMessage::Tiles { tiles, folder })?;
            }
        }
        action_id => {
            tracing::warn!(?action_id, ?context, "unknown internal action");
        }
    }
    Ok(())
}
