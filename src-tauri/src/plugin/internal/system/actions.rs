use std::{path::Path, sync::Arc};

use enigo::{Enigo, Key, Keyboard};
use serde::Deserialize;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System, UpdateKind};
use tauri_plugin_opener::{open_path, open_url};

use crate::{
    database::{DbPool, JsonObject, entity::tile::TileModel},
    device::Devices,
    events::TileInteractionContext,
    plugin::Plugins,
};

#[derive(Deserialize)]
pub struct SystemWebsiteProperties {
    url: Option<String>,
}

#[derive(Deserialize)]
pub struct SystemOpenProperties {
    path: Option<String>,
}
#[derive(Deserialize)]
pub struct SystemOpenFolderProperties {
    path: Option<String>,
}

#[derive(Deserialize)]
pub struct SystemCloseProperties {
    path: Option<String>,
}

#[derive(Deserialize)]
pub struct SystemTextProperties {
    text: Option<String>,
}

#[derive(Deserialize)]
pub struct SystemMultimediaProperties {
    action: Option<MultimediaAction>,
}

#[derive(Deserialize)]
pub struct HotkeyProperties {
    keys: Option<Keys>,
}

#[derive(Deserialize)]
pub struct Keys {
    modifiers: Vec<HotKey>,
    keys: Vec<HotKey>,
}

#[derive(Deserialize)]
pub struct HotKey {
    code: u32,
}

#[derive(Deserialize)]
pub enum MultimediaAction {
    PlayPause,
    Play,
    Pause,
    NextTrack,
    PreviousTrack,
    VolumeUp,
    VolumeDown,
    Mute,
}

pub async fn handle(
    devices: &Devices,
    plugins: &Plugins,
    context: TileInteractionContext,
    properties: JsonObject,
) -> anyhow::Result<()> {
    match context.action_id.as_str() {
        "website" => {
            let data: SystemWebsiteProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;
            if let Some(url) = data.url {
                open_url(url, None::<&str>)?;
            }
        }
        "open" => {
            let data: SystemOpenProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;
            if let Some(path) = data.path {
                open_path(path, None::<&str>)?;
            }
        }
        "open_folder" => {
            let data: SystemOpenFolderProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;
            if let Some(path) = data.path {
                open_path(path, None::<&str>)?;
            }
        }
        "close" => {
            let data: SystemCloseProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;
            if let Some(path) = data.path {
                let path = Path::new(&path);
                let mut system = System::new_with_specifics(RefreshKind::nothing().with_processes(
                    ProcessRefreshKind::nothing().with_exe(UpdateKind::OnlyIfNotSet),
                ));
                system.refresh_processes(ProcessesToUpdate::All, true);

                for process in system.processes().values() {
                    if let Some(exe_path) = process.exe() {
                        if exe_path == path {
                            tracing::debug!(?process, ?exe_path, "stopping program at path");
                            let _ = process.kill();
                        }
                    }
                }
            }
        }
        "text" => {
            let data: SystemTextProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;
            if let Some(text) = data.text {
                let mut enigo = Enigo::new(&enigo::Settings::default()).unwrap();
                // Enter text
                enigo.text(&text);
            }
        }
        "multimedia" => {
            let data: SystemMultimediaProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;

            let action = match data.action {
                Some(value) => value,
                None => return Ok(()),
            };

            let mut enigo = Enigo::new(&enigo::Settings::default()).unwrap();

            match action {
                MultimediaAction::PlayPause => {
                    enigo.key(Key::MediaPlayPause, enigo::Direction::Click);
                }
                MultimediaAction::Play => {
                    enigo.key(Key::Play, enigo::Direction::Click);
                }
                MultimediaAction::Pause => {
                    enigo.key(Key::Pause, enigo::Direction::Click);
                }
                MultimediaAction::NextTrack => {
                    enigo.key(Key::MediaNextTrack, enigo::Direction::Click);
                }
                MultimediaAction::PreviousTrack => {
                    enigo.key(Key::MediaPrevTrack, enigo::Direction::Click);
                }
                MultimediaAction::VolumeUp => {
                    enigo.key(Key::VolumeUp, enigo::Direction::Click);
                }
                MultimediaAction::VolumeDown => {
                    enigo.key(Key::VolumeDown, enigo::Direction::Click);
                }
                MultimediaAction::Mute => {
                    enigo.key(Key::VolumeMute, enigo::Direction::Click);
                }
            }
        }
        "hotkey" => {
            let data: HotkeyProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;

            let keys = match data.keys {
                Some(value) => value,
                None => return Ok(()),
            };

            let mut enigo = Enigo::new(&enigo::Settings::default()).unwrap();

            for key in &keys.modifiers {
                let key = Key::Other(key.code);
                enigo.key(key, enigo::Direction::Press);
            }

            for key in keys.keys {
                let key = Key::Other(key.code);
                enigo.key(key, enigo::Direction::Click);
            }

            for key in keys.modifiers {
                let key = Key::Other(key.code);
                enigo.key(key, enigo::Direction::Release);
            }
        }
        action_id => {
            tracing::warn!(?action_id, ?context, "unknown internal action");
        }
    }

    Ok(())
}
