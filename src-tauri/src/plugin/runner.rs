use std::{
    path::{Path, PathBuf},
    process::Stdio,
};

use serde::Serialize;
use tokio::{
    process::{Child, Command},
    sync::mpsc,
    task::AbortHandle,
};

use super::{
    manifest::{
        platform_arch, platform_os, Arch, ManifestBin, ManifestBinNative, OperatingSystem, PluginId,
    },
    Plugin, Plugins,
};

#[derive(Debug, Clone, Serialize)]
pub enum PluginTaskState {
    // Not started yet
    NotStarted,

    // Process is starting
    Starting,

    // Task not available for the current os/arch/binary
    Unavailable,

    /// Process is running
    Running {
        #[serde(skip)]
        abort: AbortHandle,
    },

    /// Plugin task ended with an error itself
    Error,

    /// Plugin task ended without an error
    Stopped,
}

pub enum PluginTaskType {
    Node,
    Native,
}

pub fn spawn_native_task(
    plugins: Plugins,

    plugin_path: PathBuf,
    exe: String,

    connect_url: String,
    plugin_id: PluginId,
) {
    let exe_path = plugin_path.join(&exe);

    // Exe does not exist
    if !exe_path.exists() {
        plugins.set_plugin_task(&plugin_id, PluginTaskState::Unavailable);
        return;
    }

    // Starting the exe
    plugins.set_plugin_task(&plugin_id, PluginTaskState::Starting);

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
            plugins.set_plugin_task(&plugin_id, PluginTaskState::Error);
            return;
        }
    };

    let abort_handle = tokio::spawn({
        let plugins = plugins.clone();
        let plugin_id = plugin_id.clone();

        async move {
            let output = match child.wait_with_output().await {
                Ok(child) => child,
                Err(cause) => {
                    tracing::error!(?cause, "failed to get plugin output");
                    plugins.set_plugin_task(&plugin_id, PluginTaskState::Error);
                    return;
                }
            };

            if output.status.success() {
                plugins.set_plugin_task(&plugin_id, PluginTaskState::Stopped);
            } else {
                plugins.set_plugin_task(&plugin_id, PluginTaskState::Error);
            }
        }
    })
    .abort_handle();

    plugins.set_plugin_task(
        &plugin_id,
        PluginTaskState::Running {
            abort: abort_handle,
        },
    );
}
