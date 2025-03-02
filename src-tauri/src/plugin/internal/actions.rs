use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    database::{entity::tile::TileModel, DbPool},
    events::{AppEventSender, PluginMessageContext},
    plugin::PluginRegistry,
};

pub async fn handle_internal_action(
    plugins: &PluginRegistry,
    app_tx: &AppEventSender,
    db: &DbPool,

    context: PluginMessageContext,
    message: serde_json::Value,
) -> anyhow::Result<()> {
    let tile = TileModel::get_by_id(db, context.tile_id)
        .await?
        .context("tile instance not found")?;

    match context.plugin_id.as_str() {
        "com.tilepad.system.navigation" => {
            handle_internal_navigation(plugins, app_tx, db, &tile, context, message).await?;
        }

        plugin_id => {
            tracing::warn!(?plugin_id, ?context, "unknown internal action");
        }
    }

    Ok(())
}

async fn handle_internal_navigation(
    plugins: &PluginRegistry,
    app_tx: &AppEventSender,
    db: &DbPool,
    tile: &TileModel,

    context: PluginMessageContext,
    message: serde_json::Value,
) -> anyhow::Result<()> {
    Ok(())
}
