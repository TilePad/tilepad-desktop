use std::{
    collections::HashMap,
    os::windows::fs::FileTypeExt,
    path::{Path, PathBuf},
    sync::Arc,
};

use garde::Validate;
use parking_lot::RwLock;
use serde::Serialize;
use tilepad_manifest::icons::{Icon, IconPackId, Manifest as IconPackManifest};

#[derive(Clone)]
pub struct Icons {
    inner: Arc<IconsInner>,
}

impl Icons {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    pub fn insert_packs(&self, plugins: Vec<IconPack>) {
        self.inner.packs.write().extend(
            plugins
                .into_iter()
                .map(|plugin| (plugin.manifest.icons.id.clone(), plugin)),
        );
    }

    pub fn get_icon_packs(&self) -> Vec<IconPack> {
        self.inner.packs.read().values().cloned().collect()
    }

    pub fn get_pack_path(&self, pack_id: &IconPackId) -> Option<PathBuf> {
        self.inner
            .packs
            .read()
            .get(pack_id)
            .map(|pack| pack.path.clone())
    }
}

#[derive(Default)]
struct IconsInner {
    /// Collection of currently loaded plugins
    packs: RwLock<HashMap<IconPackId, IconPack>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct IconPack {
    pub path: PathBuf,
    pub manifest: IconPackManifest,
    pub icons: Vec<Icon>,
}

pub async fn load_icon_packs(path: &Path) -> anyhow::Result<Vec<IconPack>> {
    let mut packs = Vec::new();
    let mut dir = tokio::fs::read_dir(path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let file_type = entry.file_type().await?;
        if !file_type.is_dir() && !file_type.is_symlink_dir() {
            continue;
        }

        if let Ok(pack) = load_icon_pack(&path).await {
            packs.push(pack);
        }
    }

    Ok(packs)
}

pub async fn load_icon_pack(path: &Path) -> anyhow::Result<IconPack> {
    let manifest_path = path.join("manifest.toml");
    let manifest = match load_manifest(&manifest_path).await {
        Ok(value) => value,
        Err(cause) => {
            tracing::error!(?cause, ?manifest_path, "failed to load manifest file");
            return Err(cause);
        }
    };

    let icons_path = path.join("icons.json");
    let icons = match load_icons(&icons_path).await {
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

pub async fn load_manifest(path: &Path) -> anyhow::Result<IconPackManifest> {
    let data = tokio::fs::read_to_string(path).await?;
    let manifest: IconPackManifest = toml::from_str(&data)?;
    manifest.validate()?;
    Ok(manifest)
}

pub async fn load_icons(path: &Path) -> anyhow::Result<Vec<Icon>> {
    let data = tokio::fs::read_to_string(path).await?;
    let icons: Vec<Icon> = serde_json::from_str(&data)?;
    icons.validate()?;
    Ok(icons)
}

pub async fn load_icon_packs_into_registry(registry: Icons, path: PathBuf) {
    let packs = match load_icon_packs(&path).await {
        Ok(value) => value,
        Err(cause) => {
            tracing::error!(?cause, ?path, "failed to load icon packs for registry");
            return;
        }
    };

    let pack_ids: Vec<IconPackId> = packs
        .iter()
        .map(|pack| (pack.manifest.icons.id.clone()))
        .collect();

    tracing::debug!(?pack_ids, "loaded icon packs");

    registry.insert_packs(packs);
}
