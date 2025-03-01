use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use garde::Validate;
use manifest::{Manifest, PluginId};
use parking_lot::RwLock;

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
    path: PathBuf,
    manifest: Manifest,
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
