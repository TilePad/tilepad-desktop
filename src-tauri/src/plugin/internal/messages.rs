use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    database::{
        entity::{folder::FolderModel, tile::TileModel},
        DbPool,
    },
    events::{AppEvent, AppEventSender, PluginAppEvent, InspectorContext},
    plugin::PluginRegistry,
};

pub async fn handle_internal_send_message(
    plugins: &PluginRegistry,
    app_tx: &AppEventSender,
    db: &DbPool,

    context: InspectorContext,
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

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NavigationInspectorMessage {
    GetFolderOptions {},
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NavigationPluginMessage {
    FolderOptions { options: Vec<FolderOption> },
}

#[derive(Serialize)]
pub struct FolderOption {
    pub label: String,
    pub value: String,
}

async fn handle_internal_navigation(
    _plugins: &PluginRegistry,
    app_tx: &AppEventSender,
    db: &DbPool,
    _tile: &TileModel,

    context: InspectorContext,
    message: serde_json::Value,
) -> anyhow::Result<()> {
    let message: NavigationInspectorMessage = serde_json::from_value(message)?;

    match message {
        NavigationInspectorMessage::GetFolderOptions {} => {
            let folders = FolderModel::all(db, context.profile_id).await?;
            let options = folders
                .into_iter()
                .map(|folder| FolderOption {
                    label: folder.name,
                    value: folder.id.to_string(),
                })
                .collect();

            let message = NavigationPluginMessage::FolderOptions { options };
            let message = serde_json::to_value(message)?;
            app_tx.send(AppEvent::Plugin(PluginAppEvent::RecvPluginMessage {
                context,
                message,
            }))?;
        }
    }

    Ok(())
}
