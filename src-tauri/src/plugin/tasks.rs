use std::{collections::HashMap, path::PathBuf, sync::Arc};

use parking_lot::RwLock;
use tilepad_manifest::plugin::{
    platform_arch, platform_os, Manifest as PluginManifest, ManifestBin, ManifestBinNative,
    PluginId,
};

use crate::plugin::runner::{self, spawn_native_task};

use super::{runner::PluginTaskState, Plugin};

#[derive(Clone, Default)]
pub struct PluginTasks {
    inner: Arc<PluginTasksInner>,
}

#[derive(Default)]
struct PluginTasksInner {
    /// Mapping for the current plugin tasks
    tasks: RwLock<HashMap<PluginId, PluginTaskState>>,
}

impl PluginTasks {
    pub fn get_states(&self) -> Vec<(PluginId, PluginTaskState)> {
        self.inner
            .tasks
            .read()
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }

    /// Sets the task state for a plugin by ID
    pub fn set_state(&self, plugin_id: PluginId, plugin_task: PluginTaskState) {
        self.inner.tasks.write().insert(plugin_id, plugin_task);
    }

    /// Stop a task by plugin ID
    pub async fn stop(&self, plugin_id: &PluginId) {
        let state = {
            let mut tasks = self.inner.tasks.write();
            match tasks.remove(plugin_id) {
                Some(value) => value,
                None => return,
            }
        };

        // Get the current state

        // Abort the plugin background task
        if let PluginTaskState::Running { handle } = state {
            handle.kill().await;
        }
    }

    pub async fn restart(
        &self,
        plugin_id: PluginId,
        plugin_path: PathBuf,
        manifest: &PluginManifest,
    ) {
        self.stop(&plugin_id).await;
        self.start(plugin_id, plugin_path, manifest);
    }

    pub fn start(&self, plugin_id: PluginId, plugin_path: PathBuf, manifest: &PluginManifest) {
        let plugin_id = manifest.plugin.id.clone();
        let connect_url = "ws://localhost:59371/plugins/ws".to_string();

        tracing::debug!(?plugin_id, "starting background task for plugin");

        let binary = match manifest.bin.as_ref() {
            Some(value) => value,
            None => {
                // No binary available for the plugin
                tracing::debug!(?plugin_id, "skipping starting plugin without binary");
                self.set_state(plugin_id, PluginTaskState::Unavailable);
                return;
            }
        };

        match binary {
            ManifestBin::Node { node } => todo!("node task support"),
            ManifestBin::Native { native } => {
                let binary = match Self::get_native_binary(native) {
                    Some(value) => value,
                    None => {
                        // No binary available for the plugin on the current os + arch
                        tracing::debug!(
                            ?plugin_id,
                            "skipping starting plugin without compatible native binary"
                        );
                        self.set_state(plugin_id.clone(), PluginTaskState::Unavailable);
                        return;
                    }
                };

                // No binary available for the plugin
                tracing::debug!(?plugin_id, os = ?binary.os, arch = ?binary.arch, "starting native plugin binary");

                runner::spawn_native_task(
                    self.clone(),
                    plugin_path,
                    binary.path.clone(),
                    connect_url,
                    plugin_id,
                );
            }
        }
    }

    /// Find the `native` binary compatible with the current platform
    fn get_native_binary(native: &[ManifestBinNative]) -> Option<&ManifestBinNative> {
        let os = platform_os();
        let arch = platform_arch();
        native.iter().find(|bin| os == bin.os && arch == bin.arch)
    }
}
