use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use action::{actions_from_plugins, Action, ActionCategory, ActionWithCategory};
use anyhow::Context;
use garde::Validate;
use manifest::{platform_arch, platform_os, ActionId, Manifest, PluginId};
use parking_lot::RwLock;
use protocol::ServerPluginMessage;
use runner::{spawn_native_task, PluginTaskState};
use serde::Serialize;
use serde_json::Map;
use socket::{PluginSessionId, PluginSessionRef};
use tauri::plugin;

use crate::{
    database::{
        entity::{plugin_properties::PluginPropertiesModel, tile::TileModel},
        DbPool,
    },
    device::Devices,
    events::{AppEventSender, InspectorContext, TileInteractionContext},
};

pub mod action;
pub mod internal;
pub mod manifest;
pub mod node;
pub mod protocol;
pub mod runner;
pub mod socket;

#[derive(Clone)]
pub struct Plugins {
    inner: Arc<PluginsInner>,
}

impl Plugins {
    pub fn new(event_tx: AppEventSender, db: DbPool) -> Self {
        Self {
            inner: Arc::new(PluginsInner {
                event_tx,
                db,
                plugins: Default::default(),
                sessions: Default::default(),
                plugin_to_session: Default::default(),
                plugin_tasks: Default::default(),
            }),
        }
    }
}

struct PluginsInner {
    /// Sender for app events
    event_tx: AppEventSender,

    /// Access to the database
    db: DbPool,

    /// Collection of currently loaded plugins
    plugins: RwLock<HashMap<PluginId, Plugin>>,

    /// Mapping for the current plugin tasks
    plugin_tasks: RwLock<HashMap<PluginId, PluginTaskState>>,

    /// Current plugin socket sessions
    sessions: RwLock<HashMap<PluginSessionId, PluginSessionRef>>,

    /// Mapping from plugin ID to session for that plugin
    plugin_to_session: RwLock<HashMap<PluginId, PluginSessionId>>,
}

#[derive(Serialize)]
pub struct PluginWithState {
    #[serde(flatten)]
    pub plugin: Plugin,
    pub state: PluginTaskState,
}

impl Plugins {
    pub fn insert_plugins(&self, plugins: Vec<Plugin>) {
        let mut plugins_map = &mut *self.inner.plugins.write();

        for plugin in plugins {
            // Spawn task for the plugin
            self.create_plugin_task(&plugin);

            // Insert the plugin into the manifest
            plugins_map.insert(plugin.manifest.plugin.id.clone(), plugin);
        }
    }

    pub fn get_plugins_with_state(&self) -> Vec<PluginWithState> {
        let plugins: Vec<Plugin> = {
            let plugins = self.inner.plugins.read();
            plugins.values().cloned().collect()
        };

        let states = self.inner.plugin_tasks.read();

        plugins
            .into_iter()
            .map(|plugin| {
                let state = states.get(&plugin.manifest.plugin.id).cloned();

                PluginWithState {
                    plugin,
                    state: state.unwrap_or(PluginTaskState::NotStarted),
                }
            })
            .collect()
    }

    pub fn get_action_collection(&self) -> Vec<ActionCategory> {
        actions_from_plugins(self.inner.plugins.read().values())
    }

    pub fn get_action(
        &self,
        plugin_id: &PluginId,
        action_id: &ActionId,
    ) -> Option<ActionWithCategory> {
        let plugins = self.inner.plugins.read();
        let plugin = plugins.get(plugin_id)?;
        let manifest_action = plugin.manifest.actions.get(action_id)?;

        Some(ActionWithCategory {
            action: Action {
                action_id: action_id.clone(),
                plugin_id: plugin_id.clone(),

                label: manifest_action.label.clone(),
                icon: manifest_action.icon.clone(),
                description: manifest_action.description.clone(),
                inspector: manifest_action.inspector.clone(),
            },
            category_label: plugin.manifest.category.label.clone(),
        })
    }

    pub fn get_plugin_path(&self, plugin_id: &PluginId) -> Option<PathBuf> {
        self.inner
            .plugins
            .read()
            .get(plugin_id)
            .map(|plugin| plugin.path.clone())
    }

    pub fn get_plugin_manifest(&self, plugin_id: &PluginId) -> Option<Manifest> {
        self.inner
            .plugins
            .read()
            .get(plugin_id)
            .map(|plugin| plugin.manifest.clone())
    }

    pub async fn handle_send_message(
        &self,

        app_tx: &AppEventSender,
        db: &DbPool,

        context: InspectorContext,
        message: serde_json::Value,
    ) -> anyhow::Result<()> {
        let manifest = self
            .get_plugin_manifest(&context.plugin_id)
            .context("plugin not found")?;

        if manifest.plugin.internal.is_some_and(|value| value) {
            internal::messages::handle_internal_send_message(self, app_tx, db, context, message)
                .await?;
        } else {
            let session = match self.get_plugin_session(&context.plugin_id) {
                Some(value) => value,
                None => return Ok(()),
            };

            session.send_message(ServerPluginMessage::RecvFromInspector {
                ctx: context,
                message,
            })?;
        }

        Ok(())
    }

    pub async fn handle_action(
        &self,
        devices: &Devices,
        db: &DbPool,
        context: TileInteractionContext,
        tile: TileModel,
    ) -> anyhow::Result<()> {
        tracing::debug!(?context, "invoking action");

        let manifest = self
            .get_plugin_manifest(&context.plugin_id)
            .context("plugin not found")?;

        if manifest.plugin.internal.is_some_and(|value| value) {
            internal::actions::handle_internal_action(self, devices, db, context, tile).await?;
        } else {
            let session = match self.get_plugin_session(&context.plugin_id) {
                Some(value) => value,
                None => return Ok(()),
            };

            session.send_message(ServerPluginMessage::TileClicked {
                ctx: context,
                properties: tile.config.properties,
            })?;
        }

        Ok(())
    }

    /// Insert a new session
    pub fn insert_session(&self, session_id: PluginSessionId, session_ref: PluginSessionRef) {
        self.inner.sessions.write().insert(session_id, session_ref);
    }

    /// Insert a new session
    pub fn get_session(&self, session_id: &PluginSessionId) -> Option<PluginSessionRef> {
        self.inner.sessions.write().get(session_id).cloned()
    }

    /// Remove a session
    pub fn remove_session(&self, session_id: PluginSessionId) {
        self.inner.sessions.write().remove(&session_id);
    }

    pub fn set_plugin_session(&self, plugin_id: PluginId, session_id: Option<PluginSessionId>) {
        if let Some(session_id) = session_id {
            self.inner
                .plugin_to_session
                .write()
                .insert(plugin_id, session_id);
        } else {
            self.inner.plugin_to_session.write().remove(&plugin_id);
        }
    }

    pub fn get_plugin_session(&self, plugin_id: &PluginId) -> Option<PluginSessionRef> {
        let session_id = {
            let plugin_to_session = self.inner.plugin_to_session.read();
            plugin_to_session.get(plugin_id).cloned()
        }?;
        let session = {
            let sessions = self.inner.sessions.read();
            sessions.get(&session_id).cloned()
        }?;
        Some(session.clone())
    }

    pub async fn get_plugin_properties(
        &self,
        plugin_id: PluginId,
    ) -> anyhow::Result<serde_json::Value> {
        let result = PluginPropertiesModel::get_by_plugin(&self.inner.db, plugin_id).await?;

        Ok(result
            .map(|value| value.properties)
            .unwrap_or_else(|| serde_json::Value::Object(Map::new())))
    }

    pub async fn set_plugin_properties(
        &self,
        plugin_id: PluginId,
        properties: serde_json::Value,
    ) -> anyhow::Result<()> {
        PluginPropertiesModel::set(&self.inner.db, plugin_id, properties).await?;
        Ok(())
    }

    pub async fn open_inspector(&self, inspector: InspectorContext) -> anyhow::Result<()> {
        if let Some(session) = self.get_plugin_session(&inspector.plugin_id) {
            _ = session.send_message(ServerPluginMessage::InspectorOpen { ctx: inspector });
        }

        Ok(())
    }

    pub async fn close_inspector(&self, inspector: InspectorContext) -> anyhow::Result<()> {
        if let Some(session) = self.get_plugin_session(&inspector.plugin_id) {
            _ = session.send_message(ServerPluginMessage::InspectorClose { ctx: inspector });
        }

        Ok(())
    }

    pub fn set_plugin_task(&self, plugin_id: &PluginId, plugin_task: PluginTaskState) {
        self.inner
            .plugin_tasks
            .write()
            .insert(plugin_id.clone(), plugin_task);
    }

    pub fn create_plugin_task(&self, plugin: &Plugin) {
        let plugin_id = plugin.manifest.plugin.id.clone();

        tracing::debug!(?plugin_id, "starting background task for plugin");

        let binary = match plugin.manifest.bin.as_ref() {
            Some(value) => value,
            None => {
                // No binary available for the plugin
                tracing::debug!(?plugin_id, "skipping starting plugin without binary");
                self.set_plugin_task(&plugin_id, PluginTaskState::Unavailable);
                return;
            }
        };

        match binary {
            manifest::ManifestBin::Node { node } => todo!("node task support"),
            manifest::ManifestBin::Native { native } => {
                let os = platform_os();
                let arch = platform_arch();

                let binary = native.iter().find(|bin| os == bin.os && arch == bin.arch);
                let binary = match binary {
                    Some(value) => value,
                    None => {
                        // No binary available for the plugin
                        tracing::debug!(
                            ?plugin_id,
                            ?os,
                            ?arch,
                            "skipping starting plugin without binary for current platform"
                        );
                        self.set_plugin_task(&plugin_id, PluginTaskState::Unavailable);
                        return;
                    }
                };

                // No binary available for the plugin
                tracing::debug!(?plugin_id, ?os, ?arch, "starting native plugin binary");

                let plugin_path = plugin.path.clone();
                let exe = binary.path.clone();

                let connect_url = "ws://localhost:59371/plugins/ws".to_string();

                spawn_native_task(self.clone(), plugin_path, exe, connect_url, plugin_id);
            }
        }
    }
}

pub async fn load_plugins_into_registry(registry: Plugins, path: PathBuf) {
    let plugins = match load_plugins(&path).await {
        Ok(value) => value,
        Err(cause) => {
            tracing::error!(?cause, ?path, "failed to load plugins for registry");
            return;
        }
    };

    tracing::debug!(?plugins, "loaded plugins");

    registry.insert_plugins(plugins);
}

#[derive(Debug, Serialize, Clone)]
pub struct Plugin {
    pub path: PathBuf,
    pub manifest: Manifest,
}

pub async fn load_plugins(path: &Path) -> anyhow::Result<Vec<Plugin>> {
    let mut plugins = Vec::new();
    let mut dir = tokio::fs::read_dir(path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let file_type = entry.file_type().await?;
        if !file_type.is_dir() {
            continue;
        }

        let manifest_path = path.join("manifest.toml");
        let manifest = match load_manifest(&manifest_path).await {
            Ok(value) => value,
            Err(cause) => {
                tracing::error!(?cause, ?manifest_path, "failed to load manifest file");
                continue;
            }
        };

        plugins.push(Plugin { path, manifest });
    }

    Ok(plugins)
}

pub async fn load_manifest(path: &Path) -> anyhow::Result<Manifest> {
    let data = tokio::fs::read_to_string(path).await?;
    let manifest: Manifest = toml::from_str(&data)?;
    manifest.validate()?;
    Ok(manifest)
}
