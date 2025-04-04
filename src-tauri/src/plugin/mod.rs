use std::{
    collections::HashMap,
    io::Cursor,
    os::windows::fs::FileTypeExt,
    path::{Path, PathBuf},
    sync::Arc,
};

use action::{actions_from_manifests, Action, ActionCategory, ActionWithCategory};
use anyhow::{anyhow, Context};
use async_zip::base::read::seek::ZipFileReader;
use garde::Validate;
use loader::{load_plugin_from_path, load_plugins_from_path, read_plugin_manifest};
use manifest::{platform_arch, platform_os, ActionId, Manifest as PluginManifest, PluginId};
use parking_lot::RwLock;
use protocol::ServerPluginMessage;
use runner::{spawn_native_task, PluginTaskState};
use serde::Serialize;
use serde_json::Map;
use socket::{PluginSessionId, PluginSessionRef};
use tasks::PluginTasks;
use tauri::plugin;
pub use tilepad_manifest::plugin as manifest;
use tokio::io::{BufReader, BufWriter};
use tokio_util::compat::FuturesAsyncReadCompatExt;

use crate::{
    database::{
        entity::{plugin_properties::PluginPropertiesModel, tile::TileModel},
        DbPool,
    },
    device::Devices,
    events::{AppEvent, AppEventSender, InspectorContext, PluginAppEvent, TileInteractionContext},
};

pub mod action;
pub mod install;
pub mod internal;
pub mod loader;
pub mod node;
pub mod protocol;
pub mod runner;
pub mod socket;
pub mod tasks;

#[derive(Clone)]
pub struct Plugins {
    inner: Arc<PluginsInner>,
}

struct PluginsInner {
    /// Sender for app events
    event_tx: AppEventSender,

    /// Access to the database
    db: DbPool,

    /// Collection of currently loaded plugins
    plugins: RwLock<HashMap<PluginId, Arc<Plugin>>>,

    /// Tasks running for plugins
    tasks: PluginTasks,

    /// Current plugin socket sessions
    sessions: RwLock<HashMap<PluginSessionId, PluginSessionRef>>,

    /// Mapping from plugin ID to session for that plugin
    plugin_to_session: RwLock<HashMap<PluginId, PluginSessionId>>,
}

#[derive(Serialize)]
pub struct PluginWithState {
    #[serde(flatten)]
    pub plugin: Arc<Plugin>,
    pub state: PluginTaskState,
}

#[derive(Debug, Serialize, Clone)]
pub struct Plugin {
    pub path: PathBuf,
    pub manifest: PluginManifest,
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
                tasks: Default::default(),
            }),
        }
    }

    pub fn tasks(&self) -> &PluginTasks {
        &self.inner.tasks
    }

    /// Load in bulk many plugins from `plugins`
    pub async fn load_plugins(&self, plugins: Vec<Plugin>) {
        for plugin in plugins {
            self.load_plugin(plugin).await;
        }
    }

    /// Load a single plugin
    pub async fn load_plugin(&self, plugin: Plugin) {
        let plugin_id = plugin.manifest.plugin.id.clone();
        let plugin_path = plugin.path.clone();

        let tasks = self.tasks();

        // Stop any existing plugin tasks for the matching plugin ID
        tasks.stop(&plugin_id).await;

        // Start a new task for the plugin
        tasks.start(plugin_id.clone(), plugin_path, &plugin.manifest);

        // Store the plugin
        {
            let mut plugins = &mut *self.inner.plugins.write();
            plugins.insert(plugin_id.clone(), Arc::new(plugin));
        }

        // Emit loaded event
        _ = self
            .inner
            .event_tx
            .send(AppEvent::Plugin(PluginAppEvent::PluginLoaded { plugin_id }));
    }

    /// Unloads the plugin with the provided `plugin_id`
    pub async fn unload_plugin(&self, plugin_id: &PluginId) -> Option<Arc<Plugin>> {
        // Stop any running plugin tasks
        self.tasks().stop(plugin_id).await;

        let plugin = {
            // Remove the plugin from the plugins list
            let mut plugins = &mut *self.inner.plugins.write();
            plugins.remove(plugin_id)
        };

        // Emit unloaded event
        _ = self
            .inner
            .event_tx
            .send(AppEvent::Plugin(PluginAppEvent::PluginUnloaded {
                plugin_id: plugin_id.clone(),
            }));

        plugin
    }

    /// Get a specific plugin
    pub fn get_plugin(&self, plugin_id: &PluginId) -> Option<Arc<Plugin>> {
        self.inner.plugins.read().get(plugin_id).cloned()
    }

    /// Get a list of all plugins and the state of the plugins task
    pub fn get_plugins_with_state(&self) -> Vec<PluginWithState> {
        let plugins = self.inner.plugins.read();
        let states = self.inner.tasks.get_states();

        plugins
            .iter()
            .map(|(plugin_id, plugin)| {
                let state = states
                    .iter()
                    .find_map(|(state_plugin_id, state)| {
                        if state_plugin_id == plugin_id {
                            Some(state.clone())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default();

                PluginWithState {
                    plugin: plugin.clone(),
                    state,
                }
            })
            .collect()
    }

    pub fn get_action_collection(&self) -> Vec<ActionCategory> {
        actions_from_manifests(
            self.inner
                .plugins
                .read()
                .values()
                .map(|value| &value.manifest),
        )
    }

    pub fn get_action(
        &self,
        plugin_id: &PluginId,
        action_id: &ActionId,
    ) -> Option<ActionWithCategory> {
        let plugins = self.inner.plugins.read();
        let plugin = plugins.get(plugin_id)?;
        let manifest_action = plugin.manifest.actions.0.get(action_id)?;

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

    pub fn set_plugin_session(&self, plugin_id: PluginId, session_id: PluginSessionId) {
        self.inner
            .plugin_to_session
            .write()
            .insert(plugin_id, session_id);
    }

    pub fn remove_plugin_session(&self, plugin_id: PluginId) {
        self.inner.plugin_to_session.write().remove(&plugin_id);
    }

    pub fn get_plugin_session(&self, plugin_id: &PluginId) -> Option<PluginSessionRef> {
        let session_id = {
            let plugin_to_session = self.inner.plugin_to_session.read();
            plugin_to_session.get(plugin_id).cloned()
        }?;

        self.get_session(&session_id)
    }

    pub async fn handle_send_message(
        &self,
        context: InspectorContext,
        message: serde_json::Value,
    ) -> anyhow::Result<()> {
        let plugin = self
            .get_plugin(&context.plugin_id)
            .context("plugin not found")?;

        if plugin.manifest.plugin.internal.is_some_and(|value| value) {
            internal::messages::handle_internal_send_message(
                self,
                &self.inner.event_tx,
                &self.inner.db,
                context,
                message,
            )
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
        context: TileInteractionContext,
        tile: TileModel,
    ) -> anyhow::Result<()> {
        tracing::debug!(?context, "invoking action");

        let plugin = self
            .get_plugin(&context.plugin_id)
            .context("plugin not found")?;

        if plugin.manifest.plugin.internal.is_some_and(|value| value) {
            internal::actions::handle_internal_action(self, devices, &self.inner.db, context, tile)
                .await?;
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

    /// Retrieve the plugin properties from a specific plugin
    pub async fn get_plugin_properties(
        &self,
        plugin_id: PluginId,
    ) -> anyhow::Result<serde_json::Value> {
        let result = PluginPropertiesModel::get_by_plugin(&self.inner.db, plugin_id).await?;

        Ok(result
            .map(|value| value.properties)
            .unwrap_or_else(|| serde_json::Value::Object(Map::new())))
    }

    /// Handle sending a message to the provided inspector context from
    /// a plugin session
    pub fn send_to_inspector(&self, ctx: InspectorContext, message: serde_json::Value) {
        _ = self
            .inner
            .event_tx
            .send(AppEvent::Plugin(PluginAppEvent::RecvPluginMessage {
                context: ctx,
                message,
            }));
    }

    /// Handle setting the plugin properties
    pub async fn set_plugin_properties(
        &self,
        plugin_id: PluginId,
        properties: serde_json::Value,
    ) -> anyhow::Result<()> {
        PluginPropertiesModel::set(&self.inner.db, plugin_id, properties).await?;
        Ok(())
    }

    /// Handle the inspector being opened, notify attached sessions for the
    /// inspector plugin
    pub fn open_inspector(&self, inspector: InspectorContext) {
        if let Some(session) = self.get_plugin_session(&inspector.plugin_id) {
            _ = session.send_message(ServerPluginMessage::InspectorOpen { ctx: inspector });
        }
    }

    /// Handle the inspector being closed, notify attached sessions for the
    /// inspector plugin
    pub fn close_inspector(&self, inspector: InspectorContext) {
        if let Some(session) = self.get_plugin_session(&inspector.plugin_id) {
            _ = session.send_message(ServerPluginMessage::InspectorClose { ctx: inspector });
        }
    }
}

pub async fn load_plugins_into_registry(registry: Plugins, path: PathBuf) {
    let plugins = match load_plugins_from_path(&path).await {
        Ok(value) => value,
        Err(cause) => {
            tracing::error!(?cause, ?path, "failed to load plugins for registry");
            return;
        }
    };

    tracing::debug!(?plugins, "loaded plugins");

    registry.load_plugins(plugins).await;
}
