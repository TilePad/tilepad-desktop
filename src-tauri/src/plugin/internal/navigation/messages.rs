use serde::{Deserialize, Serialize};

use crate::{
    database::{
        entity::{folder::FolderModel, profile::ProfileModel, tile::TileModel},
        DbPool,
    },
    events::{AppEvent, AppEventSender, InspectorContext, PluginAppEvent},
    plugin::Plugins,
};

/// Messages from the inspectors
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NavigationInspectorMessage {
    GetFolderOptions,
    GetProfileOptions,
}

/// Messages from the plugin
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NavigationPluginMessage {
    FolderOptions { options: Vec<SelectOption> },
    ProfileOptions { options: Vec<SelectOption> },
}

#[derive(Serialize)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
}

pub async fn handle(
    plugins: &Plugins,
    db: &DbPool,
    _tile: &TileModel,

    context: InspectorContext,
    message: serde_json::Value,
) -> anyhow::Result<()> {
    let message: NavigationInspectorMessage = serde_json::from_value(message)?;

    match message {
        NavigationInspectorMessage::GetFolderOptions => {
            let folders = FolderModel::all(db, context.profile_id).await?;
            let options = folders
                .into_iter()
                .map(|folder| SelectOption {
                    label: folder.name,
                    value: folder.id.to_string(),
                })
                .collect();

            let message = NavigationPluginMessage::FolderOptions { options };
            let message = serde_json::to_value(message)?;
            plugins.send_to_inspector(context, message);
        }
        NavigationInspectorMessage::GetProfileOptions => {
            let profiles = ProfileModel::all(db).await?;
            let options = profiles
                .into_iter()
                .map(|profile| SelectOption {
                    label: profile.name,
                    value: profile.id.to_string(),
                })
                .collect();

            let message = NavigationPluginMessage::ProfileOptions { options };
            let message = serde_json::to_value(message)?;
            plugins.send_to_inspector(context, message);
        }
    }

    Ok(())
}
