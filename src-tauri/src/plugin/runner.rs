use std::{
    path::{Path, PathBuf},
    process::Stdio,
};

use serde::{Serialize, Serializer};
use tokio::{
    process::{Child, Command},
    sync::mpsc,
    task::AbortHandle,
};

use super::{
    manifest::{
        platform_arch, platform_os, Arch, ManifestBin, ManifestBinNative, OperatingSystem, PluginId,
    },
    tasks::PluginTasks,
    Plugin, Plugins,
};

#[derive(Debug, Default, Clone)]
pub enum PluginTaskState {
    // Not started yet
    #[default]
    NotStarted,

    // Process is starting
    Starting,

    // Task not available for the current os/arch/binary
    Unavailable,

    /// Process is running
    Running {
        abort: AbortHandle,
    },

    /// Plugin task ended with an error itself
    Error,

    /// Plugin task ended without an error
    Stopped,
}

impl Serialize for PluginTaskState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PluginTaskState::NotStarted => serializer.serialize_str("NotStarted"),
            PluginTaskState::Starting => serializer.serialize_str("Starting"),
            PluginTaskState::Unavailable => serializer.serialize_str("Unavailable"),
            PluginTaskState::Running { .. } => serializer.serialize_str("Running"),
            PluginTaskState::Error => serializer.serialize_str("Error"),
            PluginTaskState::Stopped => serializer.serialize_str("Stopped"),
        }
    }
}

pub enum PluginTaskType {
    Node,
    Native,
}

pub fn spawn_native_task(
    tasks: PluginTasks,

    plugin_path: PathBuf,
    exe: String,

    connect_url: String,
    plugin_id: PluginId,
) {
    let exe_path = plugin_path.join(&exe);

    // Exe does not exist
    if !exe_path.exists() {
        tasks.set_state(plugin_id, PluginTaskState::Unavailable);
        return;
    }

    // Starting the exe
    tasks.set_state(plugin_id.clone(), PluginTaskState::Starting);

    tracing::debug!(?plugin_id, ?exe, ?plugin_path, "starting native plugin");

    let child = Command::new(exe_path)
        .current_dir(plugin_path)
        .args([
            "--connect-url",
            connect_url.as_str(),
            "--plugin-id",
            plugin_id.0.as_str(),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn();

    let child = match child {
        Ok(child) => child,
        Err(cause) => {
            tracing::error!(?cause, "failed to start plugin executable");
            tasks.set_state(plugin_id, PluginTaskState::Error);
            return;
        }
    };

    let abort_handle = tokio::spawn({
        let plugins = tasks.clone();
        let plugin_id = plugin_id.clone();

        async move {
            let output = match child.wait_with_output().await {
                Ok(child) => child,
                Err(cause) => {
                    tracing::error!(?cause, "failed to get plugin output");
                    plugins.set_state(plugin_id, PluginTaskState::Error);
                    return;
                }
            };

            if output.status.success() {
                plugins.set_state(plugin_id, PluginTaskState::Stopped);
            } else {
                plugins.set_state(plugin_id, PluginTaskState::Error);
            }
        }
    })
    .abort_handle();

    tasks.set_state(
        plugin_id,
        PluginTaskState::Running {
            abort: abort_handle,
        },
    );
}
