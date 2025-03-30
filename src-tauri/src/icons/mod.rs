use std::{collections::HashMap, path::PathBuf, sync::Arc};

use loader::load_icon_packs_from_path;
use parking_lot::RwLock;
use serde::Serialize;
use tilepad_manifest::icons::{Icon, IconPackId, Manifest as IconPackManifest};

pub mod install;
pub mod loader;

#[derive(Clone)]
pub struct Icons {
    inner: Arc<IconsInner>,
}

#[derive(Default)]
struct IconsInner {
    /// Collection of currently loaded plugins
    packs: RwLock<HashMap<IconPackId, Arc<IconPack>>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct IconPack {
    pub path: PathBuf,
    pub manifest: IconPackManifest,
    pub icons: Vec<Icon>,
}

impl Icons {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    /// Load a single plugin
    pub fn load_pack(&self, pack: IconPack) {
        let packs = &mut *self.inner.packs.write();
        self.load_pack_inner(packs, pack);
    }

    /// Load in bulk many plugins from `plugins`
    pub fn load_packs(&self, packs: Vec<IconPack>) {
        let plugins_map = &mut *self.inner.packs.write();
        for pack in packs {
            self.load_pack_inner(plugins_map, pack);
        }
    }

    /// Performs the actual plugin loading process for a specific plugin
    fn load_pack_inner(&self, packs: &mut HashMap<IconPackId, Arc<IconPack>>, pack: IconPack) {
        let pack_id = pack.manifest.icons.id.clone();

        // Store the plugin
        packs.insert(pack_id, Arc::new(pack));
    }

    pub fn get_icon_packs(&self) -> Vec<Arc<IconPack>> {
        self.inner.packs.read().values().cloned().collect()
    }

    pub fn get_pack_path(&self, pack_id: &IconPackId) -> Option<PathBuf> {
        self.inner
            .packs
            .read()
            .get(pack_id)
            .map(|pack| pack.path.clone())
    }

    pub fn unload_pack(&self, pack_id: &IconPackId) {
        self.inner.packs.write().remove(pack_id);
    }
}

pub async fn load_icon_packs_into_registry(registry: Icons, path: PathBuf) {
    let packs = match load_icon_packs_from_path(&path).await {
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

    registry.load_packs(packs);
}
