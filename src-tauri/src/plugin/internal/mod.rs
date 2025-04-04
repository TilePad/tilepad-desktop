pub mod navigation;
pub mod system;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    database::{
        entity::{folder::FolderModel, profile::ProfileModel, tile::TileModel},
        DbPool,
    },
    device::Devices,
    events::{AppEvent, AppEventSender, InspectorContext, PluginAppEvent, TileInteractionContext},
    plugin::Plugins,
};

pub async fn handle_internal_message(
    plugins: &Plugins,
    db: &DbPool,

    context: InspectorContext,
    message: serde_json::Value,
) -> anyhow::Result<()> {
    let tile = TileModel::get_by_id(db, context.tile_id)
        .await?
        .context("tile instance not found")?;

    match context.plugin_id.as_str() {
        "com.tilepad.system.navigation" => {
            navigation::messages::handle(plugins, db, &tile, context, message).await?;
        }

        "com.tilepad.system.system" => {
            system::messages::handle(plugins, db, &tile, context, message).await?;
        }

        plugin_id => {
            tracing::warn!(?plugin_id, ?context, "unknown internal action");
        }
    }

    Ok(())
}

pub async fn handle_internal_action(
    plugins: &Plugins,
    devices: &Devices,
    db: &DbPool,
    context: TileInteractionContext,
    tile: TileModel,
) -> anyhow::Result<()> {
    match context.plugin_id.as_str() {
        "com.tilepad.system.navigation" => {
            navigation::actions::handle(devices, plugins, db, context, tile).await?;
        }

        "com.tilepad.system.system" => {
            system::actions::handle(devices, plugins, db, context, tile).await?;
        }

        plugin_id => {
            tracing::warn!(?plugin_id, ?context, "unknown internal action");
        }
    }

    Ok(())
}
