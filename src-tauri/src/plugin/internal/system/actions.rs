use arboard::Clipboard;
use enigo::{Enigo, Key, Keyboard};
use serde::Deserialize;
use std::{path::Path, time::Duration};
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System, UpdateKind};
use tauri_plugin_opener::{open_path, open_url};

use crate::{database::JsonObject, events::TileInteractionContext};

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

pub async fn handle(context: TileInteractionContext, properties: JsonObject) -> anyhow::Result<()> {
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
                execute_enigo_action(EnoAction::Text { text });
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

            execute_enigo_action(EnoAction::Key { key });
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

            execute_enigo_action(EnoAction::HotKey { modifiers, keys });
        }
        "clipboard" => {
            let data: ClipboardProperties =
                serde_json::from_value(serde_json::Value::Object(properties))?;
            if let Some(text) = data.text {
                let mut clipboard = Clipboard::new()?;
                clipboard.set_text(text)?;
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

/// Executes an enigo action, on windows these can be done in an async environment
/// but on other platforms these are !Send so have to be performed in a dedicated
/// thread
fn execute_enigo_action(action: EnoAction) {
    std::thread::spawn(|| {
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
    });
}
