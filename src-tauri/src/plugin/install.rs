use anyhow::Context;
use std::{
    io::Cursor,
    path::{Path, PathBuf},
    str::FromStr,
};
use tilepad_manifest::plugin::{Manifest as PluginManifest, ManifestBin};
use tilepad_manifest::system::{Arch, OperatingSystem};
use tokio::{
    fs::{create_dir_all, remove_dir_all, remove_file},
    io::BufReader,
};

use crate::utils::zip::{create_zip_reader, extract_zip};

use super::node::{download_node, get_node_versions};

/// Removes any existing plugin data from the provided `path`
pub async fn remove_plugin_files(path: &Path) -> anyhow::Result<()> {
    // Remove old directory if present
    if path.is_symlink() || path.is_dir() {
        remove_dir_all(&path)
            .await
            .context("failed to remove old plugin data")?;
    } else if path.is_file() {
        remove_file(&path)
            .await
            .context("failed to remove file at plugin output path")?;
    }

    Ok(())
}

/// Installs the plugin zip from `data` saving the plugin contents to `path`
pub async fn install_plugin_zip(data: &[u8], path: &Path) -> anyhow::Result<()> {
    // Create output directory
    create_dir_all(path)
        .await
        .context("failed to create plugin directory")?;

    // Create zip reader from the data
    let reader = BufReader::new(Cursor::new(data));
    let zip = create_zip_reader(reader).await?;

    // Extract zip contents to plugin folder
    extract_zip(zip, path)
        .await
        .context("failed to extract plugin file")?;

    Ok(())
}

/// Attempts to find an existing installed node runtime matching the provided
/// version range requirement
#[tracing::instrument]
pub async fn get_node_runtime(
    runtimes_path: &Path,
    desired: &node_semver::Range,
) -> std::io::Result<Option<PathBuf>> {
    if !runtimes_path.exists() {
        tracing::debug!("no installed node runtimes");
        return Ok(None);
    }

    let mut runtime_path_entries = tokio::fs::read_dir(runtimes_path).await?;
    while let Some(entry) = runtime_path_entries.next_entry().await? {
        let metadata = match entry.metadata().await {
            Ok(value) => value,
            Err(cause) => {
                tracing::error!(?cause, "failed to get directory metadata");
                continue;
            }
        };

        // Skip non directories
        if !metadata.is_dir() {
            continue;
        }

        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        let version = match name.strip_prefix("node-") {
            Some(value) => value,
            // Not a node runtime directory
            None => continue,
        };

        let version = match node_semver::Version::from_str(version) {
            Ok(value) => value,
            Err(cause) => {
                tracing::warn!(
                    ?cause,
                    ?version,
                    "invalid node version in node runtime path"
                );
                continue;
            }
        };

        // We already have a runtime installed that satisfies the version requirement
        if version.satisfies(desired) {
            tracing::debug!(?path, "found requested node runtime");
            return Ok(Some(path));
        }
    }

    tracing::debug!("no matching node runtimes found");
    Ok(None)
}

pub async fn install_plugin_requirements(
    manifest: &PluginManifest,
    runtimes_path: &Path,
) -> anyhow::Result<()> {
    let binary = match &manifest.bin {
        Some(value) => value,
        // No requirements
        None => return Ok(()),
    };

    let node = match binary {
        ManifestBin::Node { node } => node,
        // Native binary does not need installing
        ManifestBin::Native { .. } => return Ok(()),
    };

    let desired_version = &node.version;

    // Check if we already have a runtime installed
    if get_node_runtime(runtimes_path, &desired_version.0)
        .await
        .context("checking existing node runtimes")?
        .is_some()
    {
        return Ok(());
    }

    let client = reqwest::Client::new();

    // Request available node versions
    let versions = get_node_versions(&client)
        .await
        .context("getting available node versions")?;

    let matching = versions
        .into_iter()
        .find(|version| version.version.satisfies(&desired_version.0))
        .context("failed to find a node runtime version compatible with the plugin")?;

    let output_path_name = format!("node-{}", matching.version);
    let output_path = runtimes_path.join(output_path_name);
    if !output_path.exists() {
        create_dir_all(&output_path)
            .await
            .context("creating output directory")?;
    }

    let os = OperatingSystem::default();
    let arch = Arch::default();
    download_node(&client, output_path, matching.version, os, arch)
        .await
        .context("downloading node runtime")?;

    Ok(())
}
