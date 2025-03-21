use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use action::{actions_from_plugins, Action, ActionCategory, ActionWithCategory};
use anyhow::Context;
use garde::Validate;
use manifest::{ActionId, Manifest, PluginId};
use parking_lot::RwLock;
use serde_json::Map;
use socket::{PluginSessionId, PluginSessionRef};

use crate::{
    database::{
        entity::{plugin_properties::PluginPropertiesModel, tile::TileModel},
        DbPool,
    },
    device::Devices,
    events::{AppEventSender, TileInteractionContext, InspectorContext},
};

pub mod action;
pub mod internal;
pub mod manifest;
pub mod node;
pub mod protocol;
pub mod runner;
pub mod socket;

#[derive(Clone)]
pub struct PluginRegistry {
    inner: Arc<PluginRegistryInner>,
}

impl PluginRegistry {
    pub fn new(event_tx: AppEventSender, db: DbPool) -> Self {
        Self {
            inner: Arc::new(PluginRegistryInner {
                event_tx,
                db,
                plugins: Default::default(),
                sessions: Default::default(),
                plugin_to_session: Default::default(),
            }),
        }
    }
}

struct PluginRegistryInner {
    /// Sender for app events
    event_tx: AppEventSender,

    /// Access to the database
    db: DbPool,

    /// Collection of currently loaded plugins
    plugins: RwLock<HashMap<PluginId, Plugin>>,

    /// Current plugin socket sessions
    sessions: RwLock<HashMap<PluginSessionId, PluginSessionRef>>,

    /// Mapping from plugin ID to session for that plugin
    plugin_to_session: RwLock<HashMap<PluginId, PluginSessionId>>,
}

impl PluginRegistry {
    pub fn insert_plugins(&self, plugins: Vec<Plugin>) {
        self.inner.plugins.write().extend(
            plugins
                .into_iter()
                .map(|plugin| (plugin.manifest.plugin.id.clone(), plugin)),
        );
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

            session.send_message(protocol::ServerPluginMessage::RecvFromInspector {
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

            session.send_message(protocol::ServerPluginMessage::TileClicked {
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
}

pub async fn load_plugins_into_registry(registry: PluginRegistry, path: PathBuf) {
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

#[derive(Debug)]
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
