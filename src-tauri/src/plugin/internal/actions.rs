use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    database::{entity::tile::TileModel, DbPool},
    events::{AppEventSender, PluginMessageContext},
    plugin::PluginRegistry,
};

pub async fn handle_internal_action(
    plugins: &PluginRegistry,
    db: &DbPool,
    context: PluginMessageContext,
    tile: TileModel,
) -> anyhow::Result<()> {
    match context.plugin_id.as_str() {
        "com.tilepad.system.navigation" => {
            handle_internal_navigation(plugins, db, context, tile).await?;
        }

        plugin_id => {
            tracing::warn!(?plugin_id, ?context, "unknown internal action");
        }
    }

    Ok(())
}

async fn handle_internal_navigation(
    plugins: &PluginRegistry,
    db: &DbPool,
    context: PluginMessageContext,
    tile: TileModel,
) -> anyhow::Result<()> {
    match context.action_id.as_str() {
        "switch_folder" => {}
        action_id => {
            tracing::warn!(?action_id, ?context, "unknown internal action");
        }
    }
    Ok(())
}
