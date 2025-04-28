use anyhow::Context;
use std::{io::Cursor, path::Path};
use tilepad_manifest::plugin::Manifest as PluginManifest;
use tokio::io::BufReader;

use crate::utils::zip::{create_zip_reader, extract_zip_file};

use super::Plugin;

/// Reads a plugin manifest from the provided file `path``
pub async fn read_plugin_manifest(path: &Path) -> anyhow::Result<PluginManifest> {
    let data = tokio::fs::read_to_string(path).await?;
    let manifest: PluginManifest = PluginManifest::parse(&data)?;
    Ok(manifest)
}

/// Reads a plugin manifest from the provided `bytes`
pub fn read_plugin_manifest_bytes(bytes: Vec<u8>) -> anyhow::Result<PluginManifest> {
    let data = String::from_utf8(bytes).context("manifest file bytes are not valid utf8")?;
    let manifest: PluginManifest = PluginManifest::parse(&data)?;
    Ok(manifest)
}

/// Loads a plugin from the provided `path` reads the manifest file
/// returning the loaded [Plugin]
pub async fn load_plugin_from_path(path: &Path) -> anyhow::Result<Plugin> {
    let manifest_path = path.join("manifest.json");
    let manifest = match read_plugin_manifest(&manifest_path).await {
        Ok(value) => value,
        Err(cause) => {
            tracing::error!(?cause, ?manifest_path, "failed to load manifest file");
            return Err(cause);
        }
    };

    Ok(Plugin {
        path: path.to_path_buf(),
        manifest,
    })
}

/// Loads all plugins found in the provided `path`
pub async fn load_plugins_from_path(path: &Path) -> anyhow::Result<Vec<Plugin>> {
    if !path.exists() {
        return Ok(vec![]);
    }

    let mut plugins = Vec::new();
    let mut dir = tokio::fs::read_dir(path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let metadata = tokio::fs::metadata(&path).await?;

        // Ignore anything thats not a directory
        if !metadata.is_dir() {
            continue;
        }

        // Skip directories that don't contain a manifest
        let manifest_path = path.join("manifest.json");
        if !manifest_path.exists() {
            continue;
        }

        if let Ok(plugin) = load_plugin_from_path(&path).await {
            plugins.push(plugin);
        }
    }

    Ok(plugins)
}

/// Reads the plugin manifest file from its zip
pub async fn read_plugin_manifest_zip(data: &[u8]) -> anyhow::Result<PluginManifest> {
    let reader = BufReader::new(Cursor::new(data));
    let zip = create_zip_reader(reader).await?;

    let manifest_data = extract_zip_file(zip, "manifest.json")
        .await?
        .context("plugin missing manifest.json")?;

    read_plugin_manifest_bytes(manifest_data)
}
