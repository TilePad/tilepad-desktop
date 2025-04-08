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
            tile::TileModel,
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
    db: &DbPool,
    context: TileInteractionContext,
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

            device
                .update(
                    db,
                    UpdateDevice {
                        folder_id: Some(folder_id),
                        ..Default::default()
                    },
                )
                .await?;

            if let Some(session) = devices.get_session_by_device(context.device_id) {
                session.send_message(ServerDeviceMessage::Tiles { tiles, folder });
            }
        }
        "switch_profile" => {
            let device = DeviceModel::get_by_id(db, context.device_id)
                .await?
                .context("device not found")?;
            let data: SwitchProfileProperties = serde_json::from_value(tile.config.properties)?;

            let profile_id = data.profile;
            let profile = ProfileModel::get_by_id(db, profile_id)
                .await?
                .context("unknown profile")?;
            let folder = FolderModel::get_default(db, profile_id)
                .await?
                .context("unknown folder")?;
            let tiles = TileModel::get_by_folder(db, folder.id).await?;

            device
                .update(
                    db,
                    UpdateDevice {
                        profile_id: Some(profile_id),
                        folder_id: Some(folder.id),
                        ..Default::default()
                    },
                )
                .await?;

            if let Some(session) = devices.get_session_by_device(context.device_id) {
                _ = session.send_message(ServerDeviceMessage::Tiles { tiles, folder });
            }
        }
        action_id => {
            tracing::warn!(?action_id, ?context, "unknown internal action");
        }
    }
    Ok(())
}
