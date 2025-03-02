use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use action::{actions_from_plugins, Action, ActionCategory};
use anyhow::Context;
use garde::Validate;
use manifest::{ActionId, Manifest, PluginId};
use parking_lot::RwLock;

use crate::{
    database::DbPool,
    events::{AppEventSender, PluginMessageContext},
};

pub mod action;
pub mod internal;
pub mod manifest;
pub mod node;
pub mod runner;
pub mod socket;

#[derive(Default, Clone)]
pub struct PluginRegistry {
    inner: Arc<PluginRegistryInner>,
}

#[derive(Default)]
struct PluginRegistryInner {
    /// Collection of currently loaded plugins
    plugins: RwLock<HashMap<PluginId, Plugin>>,
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

    pub fn get_action(&self, plugin_id: &PluginId, action_id: &ActionId) -> Option<Action> {
        let plugins = self.inner.plugins.read();
        let plugin = plugins.get(plugin_id)?;
        let manifest_action = plugin.manifest.actions.get(action_id)?;

        Some(Action {
            action_id: action_id.clone(),
            plugin_id: plugin_id.clone(),

            label: manifest_action.label.clone(),
            icon: manifest_action.icon.clone(),
            description: manifest_action.description.clone(),
            inspector: manifest_action.inspector.clone(),
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

        context: PluginMessageContext,
        message: serde_json::Value,
    ) -> anyhow::Result<()> {
        let manifest = self
            .get_plugin_manifest(&context.plugin_id)
            .context("plugin not found")?;

        if manifest.plugin.internal.is_some_and(|value| value) {
            internal::messages::handle_internal_send_message(self, app_tx, db, context, message)
                .await?;
        } else {
            // Pass to plugin
        }

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
