use anyhow::{bail, Context};
use std::{
    io::Cursor,
    path::{Path, PathBuf},
};
use tilepad_manifest::plugin::Manifest as PluginManifest;
use tokio::{
    fs::{create_dir_all, remove_dir_all, remove_file},
    io::BufReader,
};

use crate::utils::zip::{create_zip_reader, extract_zip};

use super::loader::read_plugin_manifest_zip;

/// Installs the plugin zip from `data` saving the plugin data to its directory
/// within `out_path`
pub async fn install_plugin(data: Vec<u8>, out_path: PathBuf) -> anyhow::Result<()> {
    // Read the plugin manifest from within the zip file
    let manifest = read_plugin_manifest_zip(&data).await?;

    // Determine plugin install directory
    let plugin_id = manifest.plugin.id;
    let directory_name = format!("{plugin_id}.tilepadPlugin");
    let path = out_path.join(directory_name);

    // Cleanup old files
    remove_existing_plugin(&path).await?;

    // Install the plugin zip file
    install_plugin_zip(&data, &path).await?;

    Ok(())
}

/// Removes any existing plugin data from the provided `path`
pub async fn remove_existing_plugin(path: &Path) -> anyhow::Result<()> {
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
