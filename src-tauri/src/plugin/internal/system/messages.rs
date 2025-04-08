use std::sync::Arc;

use anyhow::Context;
use rfd::AsyncFileDialog;
use serde::{Deserialize, Serialize};

use crate::{
    database::{entity::tile::TileModel, DbPool},
    events::InspectorContext,
    plugin::Plugins,
};

/// Messages from the inspectors
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SystemInspectorMessage {
    PickExecutableFile,
    PickFolder,
}

/// Messages from the plugin
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SystemPluginMessage {
    PickedExecutableFile { path: String },
    PickedFolder { path: String },
}

pub async fn handle(
    plugins: &Arc<Plugins>,
    db: &DbPool,
    _tile: &TileModel,

    context: InspectorContext,
    message: serde_json::Value,
) -> anyhow::Result<()> {
    let message: SystemInspectorMessage = serde_json::from_value(message)?;

    match message {
        SystemInspectorMessage::PickExecutableFile => {
            let file = AsyncFileDialog::new().pick_file().await;

            if let Some(file) = file {
                let path = file.path().to_str().context("unsupported file path")?;
                let message = SystemPluginMessage::PickedExecutableFile {
                    path: path.to_string(),
                };
                let message = serde_json::to_value(message)?;
                plugins.send_to_inspector(context, message);
            }
        }
        SystemInspectorMessage::PickFolder => {
            let path = AsyncFileDialog::new().pick_folder().await;

            if let Some(path) = path {
                let path = path.path().to_str().context("unsupported file path")?;
                let message = SystemPluginMessage::PickedFolder {
                    path: path.to_string(),
                };
                let message = serde_json::to_value(message)?;
                plugins.send_to_inspector(context, message);
            }
        }
    }

    Ok(())
}
