pub mod navigation;
pub mod system;

use std::sync::Arc;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    database::{
        DbPool, JsonObject,
        entity::{folder::FolderModel, profile::ProfileModel, tile::TileModel},
    },
    device::Devices,
    events::{AppEvent, AppEventSender, InspectorContext, PluginAppEvent, TileInteractionContext},
    plugin::Plugins,
};

pub async fn handle_internal_message(
    plugins: &Arc<Plugins>,
    db: &DbPool,
    context: InspectorContext,
    message: serde_json::Value,
) -> anyhow::Result<()> {
    match context.plugin_id.as_str() {
        "com.tilepad.system.navigation" => {
            navigation::messages::handle(plugins, db, context, message).await?;
        }

        "com.tilepad.system.system" => {
            system::messages::handle(plugins, db, context, message).await?;
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
    context: TileInteractionContext,
    properties: JsonObject,
) -> anyhow::Result<()> {
    match context.plugin_id.as_str() {
        "com.tilepad.system.navigation" => {
            navigation::actions::handle(devices, plugins, context, properties).await?;
        }

        "com.tilepad.system.system" => {
            system::actions::handle(devices, plugins, context, properties).await?;
        }

        plugin_id => {
            tracing::warn!(?plugin_id, ?context, "unknown internal action");
        }
    }

    Ok(())
}
