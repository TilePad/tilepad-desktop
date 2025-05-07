use std::{
    ffi::OsStr,
    fmt::Debug,
    path::PathBuf,
    process::{ExitStatus, Stdio},
    sync::Arc,
};

use garde::rules::AsStr;
use parking_lot::Mutex;
use serde::{Serialize, Serializer};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    join,
    process::{Child, Command},
    select,
    sync::{mpsc, oneshot},
};
use tracing::instrument::WithSubscriber;

use crate::utils::tracing::create_plugin_logger;

use super::{Plugins, manifest::PluginId};

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

/// Spawns a child process that represents a plugin task being
/// executed by the server
#[tracing::instrument(name = "spawn_child_task", skip(on_state_change))]
pub fn spawn_child_task<I, S, F>(
    exe_path: PathBuf,
    working_dir: PathBuf,
    logs_path: PathBuf,
    args: I,
    on_state_change: F,
) -> Arc<Mutex<PluginTaskState>>
where
    I: IntoIterator<Item = S> + Debug,
    S: AsRef<OsStr>,
    F: Fn(PluginTaskState) + Send + 'static,
{
    let state = Arc::new(Mutex::new(PluginTaskState::NotStarted));

    // Exe does not exist
    if !exe_path.exists() {
        on_state_change(PluginTaskState::Unavailable);
        return state;
    }

    // Starting the exe
    on_state_change(PluginTaskState::Starting);

    let mut cmd = Command::new(exe_path);

    // Windows creation flag to prevent showing windows for the process
    // (CREATE_NO_WINDOW https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)
    #[cfg(windows)]
    cmd.creation_flags(0x08000000);

    let child = cmd
        .current_dir(working_dir)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn();

    let child = match child {
        Ok(child) => child,
        Err(cause) => {
            tracing::error!(?cause, "failed to start plugin executable");
            on_state_change(PluginTaskState::Error);
            return state;
        }
    };

    let (task, handle) = ChildTask::create(child, logs_path);

    // Entered running state
    on_state_change(PluginTaskState::Running { handle });

    tokio::spawn(async move {
        let status = match task.run().await {
            Ok(child) => child,
            Err(cause) => {
                tracing::error!(?cause, "failed to get plugin output");
                on_state_change(PluginTaskState::Error);
                return;
            }
        };

        if status.success() {
            on_state_change(PluginTaskState::Stopped);
        } else {
            on_state_change(PluginTaskState::Error);
        }
    });

    state
}

#[tracing::instrument(skip(plugins))]
pub fn spawn_native_task(
    plugins: Arc<Plugins>,

    plugin_path: PathBuf,
    logs_path: PathBuf,
    exe: String,

    connect_url: String,
    plugin_id: PluginId,
) {
    let exe_path = plugin_path.join(&exe);

    spawn_child_task(
        exe_path,
        plugin_path,
        logs_path,
        [
            "--connect-url",
            connect_url.as_str(),
            "--plugin-id",
            plugin_id.0.as_str(),
        ],
        {
            let plugin_id = plugin_id.clone();
            move |task| plugins.set_task_state(plugin_id.clone(), task)
        },
    );
}

#[tracing::instrument(skip(plugins))]
pub fn spawn_node_task(
    plugins: Arc<Plugins>,

    node_path: PathBuf,
    plugin_path: PathBuf,
    logs_path: PathBuf,
    entrypoint: String,

    connect_url: String,
    plugin_id: PluginId,
) {
    let entry_path = plugin_path.join(&entrypoint);
    let entry_path = entry_path.to_string_lossy();

    #[cfg(windows)]
    let exe_path: PathBuf = node_path.join("node.exe");
    #[cfg(not(windows))]
    let exe_path: PathBuf = node_path.join("node");

    spawn_child_task(
        exe_path,
        plugin_path,
        logs_path,
        [
            entry_path.as_str(),
            "--connect-url",
            connect_url.as_str(),
            "--plugin-id",
            plugin_id.0.as_str(),
        ],
        {
            let plugin_id = plugin_id.clone();
            move |task| plugins.set_task_state(plugin_id.clone(), task)
        },
    );
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
    pub fn create(mut child: Child, logs_path: PathBuf) -> (ChildTask, ChildTaskHandle) {
        // Take the error and output pipes
        let stdout = child.stdout.take().map(|io| BufReader::new(io).lines());
        let stderr = child.stderr.take().map(|io| BufReader::new(io).lines());
        let (subscriber, guard) = create_plugin_logger(logs_path).unwrap();

        tokio::spawn(
            async move {
                let _guard = guard;

                let stdout_future = async move {
                    let mut stdout = match stdout {
                        Some(value) => value,
                        None => return,
                    };

                    while let Ok(Some(line)) = stdout.next_line().await {
                        tracing::debug!("{line}");
                    }
                };

                let stderr_future = async move {
                    let mut stderr = match stderr {
                        Some(value) => value,
                        None => return,
                    };

                    while let Ok(Some(line)) = stderr.next_line().await {
                        tracing::error!("{line}");
                    }
                };

                join!(stdout_future, stderr_future);
            }
            .with_subscriber(subscriber),
        );

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
                            _ = self.child.kill().await;
                            _ = tx.send(());
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
