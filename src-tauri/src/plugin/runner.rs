use std::{
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    process::{ExitCode, ExitStatus, Stdio},
    task::Poll,
};

use futures::channel::oneshot;
use garde::rules::AsStr;
use serde::{Serialize, Serializer};
use tokio::{
    io::{AsyncBufReadExt, BufReader, Lines},
    process::{Child, ChildStderr, ChildStdout, Command},
    select,
    sync::mpsc,
    task::AbortHandle,
};

use super::{
    manifest::{
        platform_arch, platform_os, Arch, ManifestBin, ManifestBinNative, OperatingSystem, PluginId,
    },
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
        handle: ChildTaskHandle,
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
    plugins: Plugins,

    plugin_path: PathBuf,
    exe: String,

    connect_url: String,
    plugin_id: PluginId,
) {
    let exe_path = plugin_path.join(&exe);

    // Exe does not exist
    if !exe_path.exists() {
        plugins.set_task_state(plugin_id, PluginTaskState::Unavailable);
        return;
    }

    // Starting the exe
    plugins.set_task_state(plugin_id.clone(), PluginTaskState::Starting);

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
            plugins.set_task_state(plugin_id, PluginTaskState::Error);
            return;
        }
    };

    let (task, handle) = ChildTask::create(child);
    tokio::spawn({
        let plugins = plugins.clone();
        let plugin_id = plugin_id.clone();
        let task = task;

        async move {
            let status = match task.run().await {
                Ok(child) => child,
                Err(cause) => {
                    tracing::error!(?cause, "failed to get plugin output");
                    plugins.set_task_state(plugin_id, PluginTaskState::Error);
                    return;
                }
            };

            if status.success() {
                plugins.set_task_state(plugin_id, PluginTaskState::Stopped);
            } else {
                plugins.set_task_state(plugin_id, PluginTaskState::Error);
            }
        }
    })
    .abort_handle();

    plugins.set_task_state(plugin_id, PluginTaskState::Running { handle });
}

pub fn spawn_node_task(
    plugins: Plugins,

    node_path: PathBuf,
    plugin_path: PathBuf,
    entrypoint: String,

    connect_url: String,
    plugin_id: PluginId,
) {
    let entry_path = plugin_path.join(&entrypoint);
    let entry_path = entry_path.to_string_lossy();

    #[cfg(windows)]
    let exe_path = node_path.join("node.exe");
    #[cfg(not(windows))]
    let exe_path = todo!("node not supported on this platform");

    // Exe does not exist
    if !exe_path.exists() {
        plugins.set_task_state(plugin_id, PluginTaskState::Unavailable);
        return;
    }

    // Starting the exe
    plugins.set_task_state(plugin_id.clone(), PluginTaskState::Starting);

    tracing::debug!(
        ?plugin_id,
        ?entrypoint,
        ?plugin_path,
        "starting native plugin"
    );

    let child = Command::new(exe_path)
        .current_dir(plugin_path)
        .args([
            entry_path.as_str(),
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
            plugins.set_task_state(plugin_id, PluginTaskState::Error);
            return;
        }
    };

    let (task, handle) = ChildTask::create(child);
    tokio::spawn({
        let plugins = plugins.clone();
        let plugin_id = plugin_id.clone();
        let task = task;

        async move {
            let status = match task.run().await {
                Ok(child) => child,
                Err(cause) => {
                    tracing::error!(?cause, "failed to get plugin output");
                    plugins.set_task_state(plugin_id, PluginTaskState::Error);
                    return;
                }
            };

            if status.success() {
                plugins.set_task_state(plugin_id, PluginTaskState::Stopped);
            } else {
                plugins.set_task_state(plugin_id, PluginTaskState::Error);
            }
        }
    })
    .abort_handle();

    plugins.set_task_state(plugin_id, PluginTaskState::Running { handle });
}

#[derive(Debug, Clone)]
pub struct ChildTaskHandle {
    tx: mpsc::Sender<ChildTaskMessage>,
}

impl ChildTaskHandle {
    pub async fn kill(&self) {
        let (tx, rx) = oneshot::channel();
        if self.tx.send(ChildTaskMessage::Kill { tx }).await.is_err() {
            return;
        }
        _ = rx.await;
    }
}

enum ChildTaskMessage {
    Kill {
        // Channel to notify when the process is killed
        tx: oneshot::Sender<()>,
    },
}

pub struct ChildTask {
    child: Child,
    rx: mpsc::Receiver<ChildTaskMessage>,
}

impl ChildTask {
    pub fn create(mut child: Child) -> (ChildTask, ChildTaskHandle) {
        // Take the error and output pipes
        // let stdout = child.stdout.take().map(|io| BufReader::new(io).lines());
        // let stderr = child.stderr.take().map(|io| BufReader::new(io).lines());

        let (tx, rx) = mpsc::channel(1);

        let task = ChildTask { child, rx };

        (task, ChildTaskHandle { tx })
    }

    pub async fn run(mut self) -> std::io::Result<ExitStatus> {
        loop {
            select! {
                result = self.rx.recv() => {
                    let msg = match result {
                        Some(msg) => msg,
                        None => continue,
                    };

                    match msg {
                        ChildTaskMessage::Kill { tx } => {
                            self.child.kill().await;
                            tx.send(());
                            return Ok(ExitStatus::default())
                        },
                    }
                }

                output = self.child.wait() => {
                    return output;
                }
            }
        }
    }
}
