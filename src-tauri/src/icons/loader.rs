use anyhow::Context;
use garde::Validate;
use std::{io::Cursor, path::Path};
use tilepad_manifest::icons::{Icon, Manifest as IconsManifest};
use tokio::io::BufReader;

use crate::utils::zip::{create_zip_reader, extract_zip_file};

use super::IconPack;

/// Reads a icon pack manifest from the provided file `path``
pub async fn read_icon_pack_manifest(path: &Path) -> anyhow::Result<IconsManifest> {
    let data = tokio::fs::read_to_string(path).await?;
    let manifest: IconsManifest = IconsManifest::parse(&data)?;
    Ok(manifest)
}

/// Reads a icon pack manifest from the provided `bytes`
pub async fn read_icon_pack_manifest_bytes(bytes: Vec<u8>) -> anyhow::Result<IconsManifest> {
    let data = String::from_utf8(bytes).context("manifest file bytes are not valid utf8")?;
    let manifest: IconsManifest = IconsManifest::parse(&data)?;
    Ok(manifest)
}

/// Reads a icons JSON file from the provided path
pub async fn read_icons_from_path(path: &Path) -> anyhow::Result<Vec<Icon>> {
    let data = tokio::fs::read_to_string(path).await?;
    let icons: Vec<Icon> = serde_json::from_str(&data)?;
    icons.validate()?;
    Ok(icons)
}

/// Loads a icon pack from the provided `path` reads the manifest file
/// returning the loaded [IconPack]
pub async fn load_icon_pack_from_path(path: &Path) -> anyhow::Result<IconPack> {
    let manifest_path = path.join("manifest.json");
    let manifest = match read_icon_pack_manifest(&manifest_path).await {
        Ok(value) => value,
        Err(cause) => {
            tracing::error!(?cause, ?manifest_path, "failed to load manifest file");
            return Err(cause);
        }
    };

    let icons_path = path.join("icons.json");
    let icons = match read_icons_from_path(&icons_path).await {
        Ok(value) => value,
        Err(cause) => {
            tracing::error!(?cause, ?icons_path, "failed to load icons file");
            return Err(cause);
        }
    };

    Ok(IconPack {
        path: path.to_path_buf(),
        manifest,
        icons,
    })
}

/// Loads all icon packs found in the provided `path`
pub async fn load_icon_packs_from_path(path: &Path) -> anyhow::Result<Vec<IconPack>> {
    let mut icon_packs = Vec::new();
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

        if let Ok(icon_pack) = load_icon_pack_from_path(&path).await {
            icon_packs.push(icon_pack);
        }
    }

    Ok(icon_packs)
}

/// Reads the icon pack manifest file from its zip
pub async fn read_icon_pack_manifest_zip(data: &[u8]) -> anyhow::Result<IconsManifest> {
    let reader = BufReader::new(Cursor::new(data));
    let zip = create_zip_reader(reader).await?;

    let manifest_data = extract_zip_file(zip, "manifest.json")
        .await?
        .context("icon pack missing manifest.json")?;

    read_icon_pack_manifest_bytes(manifest_data).await
}
