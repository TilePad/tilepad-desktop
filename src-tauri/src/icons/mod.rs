use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use loader::load_icon_packs_from_path;
use parking_lot::RwLock;
use serde::Serialize;
use tilepad_manifest::icons::{Icon, IconPackId, IconsManifest as IconPackManifest};
use uuid::Uuid;

use crate::{
    database::entity::tile::TileIcon,
    events::{AppEvent, AppEventSender, IconPackAppEvent},
    utils::file::file_extension,
};

pub mod install;
pub mod loader;

pub struct Icons {
    /// Sender for app events
    event_tx: AppEventSender,

    /// Path for icon packs
    packs_path: PathBuf,

    /// Path for user uploaded icons
    uploaded_path: PathBuf,

    /// Collection of currently loaded plugins
    packs: RwLock<HashMap<IconPackId, Arc<IconPack>>>,
}

/// Pack of icons loaded from the disk
#[derive(Debug, Serialize, Clone)]
pub struct IconPack {
    pub path: PathBuf,
    pub manifest: IconPackManifest,
    pub icons: Vec<Icon>,
}

impl Icons {
    pub fn new(event_tx: AppEventSender, packs_path: PathBuf, uploaded_path: PathBuf) -> Self {
        Self {
            event_tx,
            packs_path,
            uploaded_path,
            packs: Default::default(),
        }
    }

    /// Get the path to the icon packs directory
    pub fn packs_path(&self) -> PathBuf {
        self.packs_path.clone()
    }

    /// Get the path to the user uploaded icons
    pub fn uploaded_path(&self) -> &Path {
        &self.uploaded_path
    }

    /// Loads all icon packs from the default icon pack paths
    pub async fn load_defaults(&self) {
        self.load_icon_packs_from_path(&self.packs_path).await;
    }

    /// Loads all icon packs from the provided path
    pub async fn load_icon_packs_from_path(&self, path: &Path) {
        let packs = match load_icon_packs_from_path(path).await {
            Ok(value) => value,
            Err(cause) => {
                tracing::error!(?cause, ?path, "failed to load icon packs");
                return;
            }
        };

        let pack_ids: Vec<IconPackId> = packs
            .iter()
            .map(|pack| (pack.manifest.icons.id.clone()))
            .collect();

        tracing::debug!(?pack_ids, "loaded icon packs");

        for pack in packs {
            self.load_pack(pack);
        }
    }

    /// Load a single plugin
    pub fn load_pack(&self, pack: IconPack) {
        let pack_id = pack.manifest.icons.id.clone();

        // Store the plugin
        {
            self.packs.write().insert(pack_id.clone(), Arc::new(pack));
        }

        // Emit loaded event
        _ = self
            .event_tx
            .send(AppEvent::IconPack(IconPackAppEvent::Loaded { pack_id }));
    }

    /// Get a list of all icon packs
    pub fn get_icon_packs(&self) -> Vec<Arc<IconPack>> {
        self.packs.read().values().cloned().collect()
    }

    /// Get the file system path to the files for the pack with
    /// the provided `pack_id`
    pub fn get_pack_path(&self, pack_id: &IconPackId) -> Option<PathBuf> {
        self.packs.read().get(pack_id).map(|pack| pack.path.clone())
    }

    /// Upload the specified pack
    pub fn unload_pack(&self, pack_id: &IconPackId) {
        self.packs.write().remove(pack_id);

        let pack_id = pack_id.clone();

        // Emit unloaded event
        _ = self
            .event_tx
            .send(AppEvent::IconPack(IconPackAppEvent::Unloaded { pack_id }));
    }

    /// Uploads a user generated icon to a randomly generated file path
    /// with the collection. Returns the name of the generated file
    pub async fn upload_user_icon(&self, name: String, data: Vec<u8>) -> anyhow::Result<String> {
        let uploaded_icons = &self.uploaded_path;
        if !uploaded_icons.exists() {
            tokio::fs::create_dir_all(&uploaded_icons).await?;
        }

        // Generate a unique file name
        let extension = file_extension(name)?;
        let file_id = Uuid::new_v4();
        let file_name = format!("{file_id}.{extension}");

        let file_path = uploaded_icons.join(&file_name);

        tokio::fs::write(&file_path, data)
            .await
            .context("save file")?;

        Ok(file_name)
    }

    // Handle change in icon when using an uploaded icon (Remove the old file)
    pub async fn handle_tile_change_icon(&self, previous_icon: &TileIcon) -> anyhow::Result<()> {
        let path = match previous_icon {
            TileIcon::Uploaded { path } => path,
            _ => return Ok(()),
        };

        let file_path = self.uploaded_path.join(path);
        if file_path.exists() {
            tokio::fs::remove_file(file_path).await?;
        }

        Ok(())
    }
}
