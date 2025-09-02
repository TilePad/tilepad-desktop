use anyhow::Context;
use arboard::Clipboard;
use enigo::{Enigo, Key, Keyboard};
use serde::Deserialize;
use std::{path::Path, time::Duration};
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System, UpdateKind};
use tauri_plugin_opener::{open_path, open_url};
use tokio::sync::oneshot;

use crate::{
    database::JsonObject,
    device::{Devices, protocol::DeviceIndicator},
    events::TileInteractionContext,
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

#[derive(Deserialize)]
pub struct ClipboardProperties {
    text: Option<String>,
}

pub async fn handle(
    devices: &Devices,
    context: TileInteractionContext,
    properties: JsonObject,
) -> anyhow::Result<()> {
    let success_indicator = || {
        devices.display_tile_indicator(
            context.device_id,
            context.tile_id,
            DeviceIndicator::Success,
            1000,
        );
    };

    let error_indicator = || {
        devices.display_tile_indicator(
            context.device_id,
            context.tile_id,
            DeviceIndicator::Error,
            2000,
        );
    };

    let loading_indicator = || {
        devices.display_tile_indicator(
            context.device_id,
            context.tile_id,
            DeviceIndicator::Loading,
            10_000,
        );
    };

    match context.action_id.as_str() {
        "website" => {
            let data: SystemWebsiteProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;
            if let Some(url) = data.url {
                loading_indicator();
                open_url(url, None::<&str>).inspect_err(|_| error_indicator())?;
                success_indicator();
            }
        }
        "open" => {
            let data: SystemOpenProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;
            if let Some(path) = data.path {
                loading_indicator();
                open_path(path, None::<&str>).inspect_err(|_| error_indicator())?;
                success_indicator();
            }
        }
        "open_folder" => {
            let data: SystemOpenFolderProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;
            if let Some(path) = data.path {
                loading_indicator();
                open_path(path, None::<&str>).inspect_err(|_| error_indicator())?;
                success_indicator();
            }
        }
        "close" => {
            let data: SystemCloseProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;
            if let Some(path) = data.path {
                loading_indicator();

                let path = Path::new(&path);
                let mut system = System::new_with_specifics(RefreshKind::nothing().with_processes(
                    ProcessRefreshKind::nothing().with_exe(UpdateKind::OnlyIfNotSet),
                ));
                system.refresh_processes(ProcessesToUpdate::All, true);

                for process in system.processes().values() {
                    if let Some(exe_path) = process.exe()
                        && exe_path == path
                    {
                        tracing::debug!(?process, ?exe_path, "stopping program at path");
                        let _ = process.kill();
                    }
                }

                success_indicator();
            }
        }
        "text" => {
            let data: SystemTextProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;
            if let Some(text) = data.text {
                loading_indicator();
                background_execute_enigo_action(EnoAction::Text { text })
                    .await
                    .inspect_err(|_| error_indicator())?;
                success_indicator();
            }
        }
        "multimedia" => {
            let data: SystemMultimediaProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;

            let action = match data.action {
                Some(value) => value,
                None => return Ok(()),
            };

            let key: Key = match action {
                MultimediaAction::PlayPause => Key::MediaPlayPause,
                // Actual play button only supported on windows
                #[cfg(windows)]
                MultimediaAction::Play => Key::Play,
                // Non windows targets fall back to the play pause key
                #[cfg(not(windows))]
                MultimediaAction::Play => Key::MediaPlayPause,

                // Pause button only supported on windows and linux
                #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
                MultimediaAction::Pause => Key::Pause,

                // Mac falls back to the play pause toggle
                #[cfg(target_os = "macos")]
                MultimediaAction::Pause => Key::MediaPlayPause,

                MultimediaAction::NextTrack => Key::MediaNextTrack,
                MultimediaAction::PreviousTrack => Key::MediaPrevTrack,
                MultimediaAction::VolumeUp => Key::VolumeUp,
                MultimediaAction::VolumeDown => Key::VolumeDown,
                MultimediaAction::Mute => Key::VolumeMute,
            };

            loading_indicator();
            background_execute_enigo_action(EnoAction::Key { key })
                .await
                .inspect_err(|_| error_indicator())?;
            success_indicator();
        }
        "hotkey" => {
            let data: HotkeyProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;

            let keys = match data.keys {
                Some(value) => value,
                None => return Ok(()),
            };

            let modifiers: Vec<Key> = keys
                .modifiers
                .into_iter()
                .map(|key| Key::Other(key.code))
                .collect();

            let keys: Vec<Key> = keys
                .keys
                .into_iter()
                .map(|key| Key::Other(key.code))
                .collect();

            loading_indicator();
            background_execute_enigo_action(EnoAction::HotKey { modifiers, keys })
                .await
                .inspect_err(|_| error_indicator())?;
            success_indicator();
        }
        "clipboard" => {
            let data: ClipboardProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;
            if let Some(text) = data.text {
                let mut clipboard = Clipboard::new()?;
                clipboard.set_text(text)?;
                success_indicator();
            }
        }
        action_id => {
            tracing::warn!(?action_id, ?context, "unknown internal action");
        }
    }

    Ok(())
}

pub enum EnoAction {
    HotKey { modifiers: Vec<Key>, keys: Vec<Key> },
    Key { key: Key },
    Text { text: String },
}

/// Execute a enigo action in the background on a dedicated thread,
/// waits for the response through a channel
async fn background_execute_enigo_action(action: EnoAction) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();

    std::thread::spawn(|| {
        let outcome = execute_enigo_action(action);
        _ = tx.send(outcome);
    });

    rx.await.context("channel closed")?
}

/// Executes an enigo action, on windows these can be done in an async environment
/// but on other platforms these are !Send so have to be performed in a dedicated
/// thread. Calls to this will block so use std::thread::spawn
fn execute_enigo_action(action: EnoAction) -> anyhow::Result<()> {
    let mut enigo = Enigo::new(&enigo::Settings::default())?;

    match action {
        EnoAction::HotKey { modifiers, keys } => {
            for key in &modifiers {
                enigo.key(*key, enigo::Direction::Press)?;
            }

            for key in keys {
                enigo.key(key, enigo::Direction::Click)?;
            }

            for key in modifiers {
                enigo.key(key, enigo::Direction::Release)?;
            }
        }
        EnoAction::Key { key } => {
            enigo.key(key, enigo::Direction::Click)?;
        }
        EnoAction::Text { text } => {
            let mut current_text = String::new();

            for char in text.chars() {
                match char {
                    '\n' => {
                        // Send the current buffered text
                        if !current_text.is_empty() {
                            enigo.text(&current_text)?;
                            current_text.clear();
                        }

                        // Enter new line
                        enigo.key(Key::Return, enigo::Direction::Click)?;
                    }
                    char => {
                        current_text.push(char);
                    }
                }

                // Reached enough characters to send
                if !current_text.is_empty() {
                    enigo.text(&current_text)?;
                    current_text.clear();
                }

                // Sleep between sends
                std::thread::sleep(Duration::from_millis(2));
            }
        }
    }

    Ok::<(), anyhow::Error>(())
}
