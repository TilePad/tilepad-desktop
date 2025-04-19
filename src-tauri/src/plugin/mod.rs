use std::{
    collections::HashMap,
    io::Cursor,
    os::windows::fs::FileTypeExt,
    path::{Path, PathBuf},
    sync::Arc,
};

use action::{Action, ActionCategory, ActionWithCategory, actions_from_manifests};
use anyhow::{Context, anyhow};
use async_zip::base::read::seek::ZipFileReader;
use garde::Validate;
use install::get_node_runtime;
use loader::{load_plugin_from_path, load_plugins_from_path, read_plugin_manifest};
use manifest::{ActionId, Manifest as PluginManifest, PluginId, platform_arch, platform_os};
use parking_lot::RwLock;
use protocol::ServerPluginMessage;
use runner::{PluginTaskState, spawn_native_task};
use serde::Serialize;
use serde_json::Map;
use session::{PluginSessionId, PluginSessionRef};
use tauri::plugin;
pub use tilepad_manifest::plugin as manifest;
use tilepad_manifest::plugin::{ManifestBin, ManifestBinNative};
use tokio::io::{BufReader, BufWriter};
use tokio_util::compat::FuturesAsyncReadCompatExt;

use crate::{
    database::{
        DbPool, JsonObject,
        entity::{
            plugin_properties::PluginPropertiesModel,
            tile::{TileId, TileModel},
        },
    },
    device::Devices,
    events::{
        AppEvent, AppEventSender, DeepLinkContext, InspectorContext, PluginAppEvent,
        TileInteractionContext,
    },
    icons::Icons,
};

pub mod action;
pub mod install;
pub mod internal;
pub mod loader;
pub mod node;
pub mod protocol;
pub mod runner;
pub mod session;

pub struct Plugins {
    /// Sender for app events
    event_tx: AppEventSender,

    /// Access to the database
    db: DbPool,

    /// Path to the core plugins directory
    core_path: PathBuf,

    /// Path to the user plugins directory
    user_path: PathBuf,

    /// Runtimes path
    runtimes_path: PathBuf,

    /// Collection of currently loaded plugins
    plugins: RwLock<HashMap<PluginId, Arc<Plugin>>>,

    /// Mapping for the current plugin tasks
    tasks: RwLock<HashMap<PluginId, PluginTaskState>>,

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
    pub fn new(
        event_tx: AppEventSender,
        db: DbPool,
        core_path: PathBuf,
        user_path: PathBuf,
        runtimes_path: PathBuf,
    ) -> Self {
        Self {
            event_tx,
            db,
            core_path,
            user_path,
            runtimes_path,

            plugins: Default::default(),
            sessions: Default::default(),
            plugin_to_session: Default::default(),
            tasks: Default::default(),
        }
    }

    pub fn user_path(&self) -> PathBuf {
        self.user_path.clone()
    }

    pub fn runtimes_path(&self) -> PathBuf {
        self.runtimes_path.clone()
    }

    /// Loads all icon packs from the default icon pack paths
    pub async fn load_defaults(self: &Arc<Self>) {
        // Load from the core plugins directory
        self.load_plugins_from_path(&self.core_path).await;

        // Load from the user plugins directory
        self.load_plugins_from_path(&self.user_path).await;
    }

    pub async fn load_plugins_from_path(self: &Arc<Self>, path: &Path) {
        let plugins = match load_plugins_from_path(path).await {
            Ok(value) => value,
            Err(cause) => {
                tracing::error!(?cause, ?path, "failed to load plugins");
                return;
            }
        };

        tracing::debug!(?plugins, "loaded plugins");

        self.load_plugins(plugins).await;
    }

    /// Load in bulk many plugins from `plugins`
    pub async fn load_plugins(self: &Arc<Self>, plugins: Vec<Plugin>) {
        for plugin in plugins {
            self.load_plugin(plugin).await;
        }
    }

    /// Load a single plugin
    pub async fn load_plugin(self: &Arc<Self>, plugin: Plugin) {
        let plugin_id = plugin.manifest.plugin.id.clone();
        let plugin_path = plugin.path.clone();

        // Stop any existing plugin tasks for the matching plugin ID
        self.stop_task(&plugin_id).await;

        // Start a new task for the plugin
        self.start_task(plugin_id.clone(), plugin_path, &plugin.manifest)
            .await;

        // Store the plugin
        {
            self.plugins
                .write()
                .insert(plugin_id.clone(), Arc::new(plugin));
        }

        // Emit loaded event
        _ = self
            .event_tx
            .send(AppEvent::Plugin(PluginAppEvent::Loaded { plugin_id }));
    }

    /// Unloads the plugin with the provided `plugin_id`
    pub async fn unload_plugin(&self, plugin_id: &PluginId) -> Option<Arc<Plugin>> {
        // Stop any running plugin tasks
        self.stop_task(plugin_id).await;

        let plugin = {
            // Remove the plugin from the plugins list
            self.plugins.write().remove(plugin_id)
        };

        // Emit unloaded event
        _ = self
            .event_tx
            .send(AppEvent::Plugin(PluginAppEvent::Unloaded {
                plugin_id: plugin_id.clone(),
            }));

        plugin
    }

    /// Get a specific plugin
    pub fn get_plugin(&self, plugin_id: &PluginId) -> Option<Arc<Plugin>> {
        self.plugins.read().get(plugin_id).cloned()
    }

    /// Get a list of all plugins and the state of the plugins task
    pub fn get_plugins_with_state(&self) -> Vec<PluginWithState> {
        let plugins = self.plugins.read();
        let states = self.get_task_states();

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
        actions_from_manifests(self.plugins.read().values().map(|value| &value.manifest))
    }

    pub fn get_action(
        &self,
        plugin_id: &PluginId,
        action_id: &ActionId,
    ) -> Option<ActionWithCategory> {
        let plugins = self.plugins.read();
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
        self.sessions.write().insert(session_id, session_ref);
    }

    /// Insert a new session
    pub fn get_session(&self, session_id: &PluginSessionId) -> Option<PluginSessionRef> {
        self.sessions.write().get(session_id).cloned()
    }

    /// Remove a session
    pub fn remove_session(&self, session_id: PluginSessionId, plugin_id: Option<PluginId>) {
        self.sessions.write().remove(&session_id);

        if let Some(plugin_id) = plugin_id {
            self.plugin_to_session.write().remove(&plugin_id);
        }
    }

    pub fn set_plugin_session(&self, plugin_id: PluginId, session_id: PluginSessionId) {
        self.plugin_to_session.write().insert(plugin_id, session_id);
    }

    pub fn get_plugin_session(&self, plugin_id: &PluginId) -> Option<PluginSessionRef> {
        let session_id = {
            let plugin_to_session = self.plugin_to_session.read();
            plugin_to_session.get(plugin_id).cloned()
        }?;

        self.get_session(&session_id)
    }

    pub async fn handle_send_message(
        self: &Arc<Self>,
        context: InspectorContext,
        message: serde_json::Value,
    ) -> anyhow::Result<()> {
        tracing::debug!(?context, ?message, "sending message to plugin");

        let plugin = self
            .get_plugin(&context.plugin_id)
            .context("plugin not found")?;

        if plugin.manifest.plugin.internal.is_some_and(|value| value) {
            internal::handle_internal_message(self, &self.db, context, message).await?;
        } else {
            tracing::debug!("sent message to plugin");
            let session = match self.get_plugin_session(&context.plugin_id) {
                Some(value) => value,
                None => return Ok(()),
            };

            session.send_message(ServerPluginMessage::RecvFromInspector {
                ctx: context,
                message,
            });
        }

        Ok(())
    }

    pub async fn handle_action(
        &self,
        devices: &Devices,
        ctx: TileInteractionContext,
        properties: JsonObject,
    ) -> anyhow::Result<()> {
        tracing::debug!(?ctx, ?properties, "invoking action");

        let plugin = self
            .get_plugin(&ctx.plugin_id)
            .context("plugin not found")?;

        if plugin.manifest.plugin.internal.is_some_and(|value| value) {
            internal::handle_internal_action(self, devices, ctx, properties).await?;
        } else {
            let session = match self.get_plugin_session(&ctx.plugin_id) {
                Some(value) => value,
                None => return Ok(()),
            };

            session.send_message(ServerPluginMessage::TileClicked { ctx, properties });
        }

        Ok(())
    }

    /// Retrieve the plugin properties from a specific plugin
    pub async fn get_plugin_properties(&self, plugin_id: PluginId) -> anyhow::Result<JsonObject> {
        let result = PluginPropertiesModel::get_by_plugin(&self.db, plugin_id).await?;
        Ok(result.map(|value| value.properties).unwrap_or_default())
    }

    /// Handle sending a message to the provided inspector context from
    /// a plugin session
    pub fn send_to_inspector(&self, ctx: InspectorContext, message: serde_json::Value) {
        _ = self
            .event_tx
            .send(AppEvent::Plugin(PluginAppEvent::Message {
                context: ctx,
                message,
            }));
    }

    /// Handle setting the plugin properties
    pub async fn set_plugin_properties(
        &self,
        plugin_id: PluginId,
        properties: JsonObject,
        partial: bool,
    ) -> anyhow::Result<()> {
        match partial {
            true => {
                self.set_plugin_properties_partial(plugin_id, properties)
                    .await
            }
            false => {
                self.set_plugin_properties_replace(plugin_id, properties)
                    .await
            }
        }
    }

    /// Handle setting the plugin properties
    pub async fn set_plugin_properties_replace(
        &self,
        plugin_id: PluginId,
        properties: JsonObject,
    ) -> anyhow::Result<()> {
        // Update the plugin properties
        PluginPropertiesModel::set(&self.db, plugin_id, properties).await?;
        Ok(())
    }

    /// Handle setting the plugin properties performing a partial update merging the
    /// new properties onto the previous object
    pub async fn set_plugin_properties_partial(
        &self,
        plugin_id: PluginId,
        properties: JsonObject,
    ) -> anyhow::Result<()> {
        let model = PluginPropertiesModel::get_by_plugin(&self.db, plugin_id.clone()).await?;

        // Get existing object
        let mut existing_properties = match model.map(|model| model.properties) {
            Some(object) => object,

            // Existing is missing or invalid
            _ => serde_json::Map::new(),
        };

        // Merge the new properties onto the old
        for (key, value) in properties {
            existing_properties.insert(key, value);
        }

        // Update the plugin properties
        PluginPropertiesModel::set(&self.db, plugin_id, existing_properties).await?;
        Ok(())
    }

    /// Handle the inspector being opened, notify attached sessions for the
    /// inspector plugin
    pub fn open_inspector(&self, inspector: InspectorContext) {
        if let Some(session) = self.get_plugin_session(&inspector.plugin_id) {
            session.send_message(ServerPluginMessage::InspectorOpen { ctx: inspector });
        }
    }

    /// Handle the inspector being closed, notify attached sessions for the
    /// inspector plugin
    pub fn close_inspector(&self, inspector: InspectorContext) {
        if let Some(session) = self.get_plugin_session(&inspector.plugin_id) {
            session.send_message(ServerPluginMessage::InspectorClose { ctx: inspector });
        }
    }

    pub fn deep_link(&self, plugin_id: &PluginId, ctx: DeepLinkContext) {
        if let Some(session) = self.get_plugin_session(plugin_id) {
            session.send_message(ServerPluginMessage::DeepLink { ctx });
        }
    }

    pub fn get_task_states(&self) -> Vec<(PluginId, PluginTaskState)> {
        self.tasks
            .read()
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }

    /// Sets the task state for a plugin by ID
    pub fn set_task_state(&self, plugin_id: PluginId, task_state: PluginTaskState) {
        {
            self.tasks
                .write()
                .insert(plugin_id.clone(), task_state.clone());
        }

        _ = self
            .event_tx
            .send(AppEvent::Plugin(PluginAppEvent::TaskStateChanged {
                plugin_id,
                state: task_state,
            }));
    }

    pub async fn restart_task(
        self: &Arc<Self>,
        plugin_id: PluginId,
        plugin_path: PathBuf,
        manifest: &PluginManifest,
    ) {
        self.stop_task(&plugin_id).await;
        self.start_task(plugin_id, plugin_path, manifest);
    }

    pub async fn start_task(
        self: &Arc<Self>,
        plugin_id: PluginId,
        plugin_path: PathBuf,
        manifest: &PluginManifest,
    ) {
        let plugin_id = manifest.plugin.id.clone();
        let connect_url = "ws://localhost:59371/plugins/ws".to_string();

        tracing::debug!(?plugin_id, "starting background task for plugin");

        let binary = match manifest.bin.as_ref() {
            Some(value) => value,
            None => {
                // No binary available for the plugin
                tracing::debug!(?plugin_id, "skipping starting plugin without binary");
                self.set_task_state(plugin_id, PluginTaskState::Unavailable);
                return;
            }
        };

        match binary {
            ManifestBin::Node { node } => {
                let runtime_path = match get_node_runtime(&self.runtimes_path, &node.version.0)
                    .await
                {
                    Ok(Some(value)) => value,
                    Ok(None) => {
                        // No binary available for the plugin on the current os + arch
                        tracing::debug!(?plugin_id, "skipping node plugin, runtime unavailable");
                        self.set_task_state(plugin_id.clone(), PluginTaskState::Unavailable);
                        return;
                    }
                    Err(cause) => {
                        tracing::debug!(
                            ?cause,
                            ?plugin_id,
                            "skipping node plugin, failed to find runtime"
                        );
                        self.set_task_state(plugin_id.clone(), PluginTaskState::Unavailable);
                        return;
                    }
                };
                // No binary available for the plugin
                tracing::debug!(?plugin_id, ?runtime_path, entrypoint = ?node.entrypoint, "starting node plugin");

                runner::spawn_node_task(
                    self.clone(),
                    runtime_path,
                    plugin_path,
                    node.entrypoint.clone(),
                    connect_url,
                    plugin_id,
                );
            }
            ManifestBin::Native { native } => {
                let binary = match Self::get_native_binary(native) {
                    Some(value) => value,
                    None => {
                        // No binary available for the plugin on the current os + arch
                        tracing::debug!(
                            ?plugin_id,
                            "skipping starting plugin without compatible native binary"
                        );
                        self.set_task_state(plugin_id.clone(), PluginTaskState::Unavailable);
                        return;
                    }
                };

                // No binary available for the plugin
                tracing::debug!(?plugin_id, os = ?binary.os, arch = ?binary.arch, "starting native plugin binary");

                runner::spawn_native_task(
                    self.clone(),
                    plugin_path,
                    binary.path.clone(),
                    connect_url,
                    plugin_id,
                );
            }
        }
    }

    /// Find the `native` binary compatible with the current platform
    fn get_native_binary(native: &[ManifestBinNative]) -> Option<&ManifestBinNative> {
        let os = platform_os();
        let arch = platform_arch();
        native.iter().find(|bin| os == bin.os && arch == bin.arch)
    }

    /// Stop a task by plugin ID
    pub async fn stop_task(&self, plugin_id: &PluginId) {
        // Get the current state
        let state = match self.tasks.write().remove(plugin_id) {
            Some(value) => value,
            None => return,
        };

        // Abort the plugin background task
        if let PluginTaskState::Running { handle } = state {
            handle.kill().await;
        }
    }
}
