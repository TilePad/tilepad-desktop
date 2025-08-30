use crate::utils::tracing::{PluginSubscriber, create_plugin_logger};
use garde::rules::AsStr;
use parking_lot::Mutex;
use serde::{Serialize, Serializer};
use std::{
    ffi::OsStr,
    fmt::{self, Debug},
    path::PathBuf,
    process::{ExitStatus, Stdio},
    sync::Arc,
};
use tilepad_manifest::plugin::PluginId;
use tokio::{
    fs::create_dir_all,
    io::{AsyncBufReadExt, BufReader},
    join,
    process::{Child, Command},
    select,
    sync::{mpsc, oneshot},
};
use tracing::instrument::WithSubscriber;
use tracing_appender::non_blocking::WorkerGuard;

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

pub trait TaskStateHolder: Debug + Send + 'static {
    fn on_change_state(&self, state: PluginTaskState);
}

pub struct TaskLogger {
    pub subscriber: PluginSubscriber,
    pub guard: WorkerGuard,
}

impl fmt::Debug for TaskLogger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskLogger").finish()
    }
}

pub async fn create_task_logger(logs_path: PathBuf) -> anyhow::Result<TaskLogger> {
    // Try create logging directory
    if !logs_path.exists() {
        create_dir_all(&logs_path).await?;
    }

    let (subscriber, guard) = create_plugin_logger(logs_path).unwrap();
    Ok(TaskLogger { subscriber, guard })
}

#[derive(Debug)]
pub struct TaskOptions<S: TaskStateHolder> {
    /// Connection URL for the plugin server
    pub connect_url: String,
    /// ID of the plugin the task is for
    pub plugin_id: PluginId,

    /// Path where the plugin itself is stored
    pub plugin_path: PathBuf,

    /// Logger which plugin output should be logged to
    pub logger: TaskLogger,

    /// Handler for updating the task state
    pub state_handler: S,
}

#[derive(Debug)]
pub struct NodeTaskOptions<S: TaskStateHolder> {
    /// Path where the node runtime is stored
    pub runtime_path: PathBuf,
    /// Entrypoint for the node application
    pub entrypoint: String,
    /// Options used by all tasks
    pub task: TaskOptions<S>,
}

#[derive(Debug)]
pub struct NativeTaskOptions<S: TaskStateHolder> {
    /// Entrypoint for the native application
    pub exe: String,
    /// Options used by all tasks
    pub task: TaskOptions<S>,
}

#[tracing::instrument]
pub fn spawn_native_task<S>(options: NativeTaskOptions<S>)
where
    S: TaskStateHolder,
{
    let task = options.task;
    let exe_path = task.plugin_path.join(&options.exe);

    spawn_child_task(
        exe_path,
        task.plugin_path,
        task.logger,
        [
            "--connect-url",
            task.connect_url.as_str(),
            "--plugin-id",
            task.plugin_id.0.as_str(),
        ],
        task.state_handler,
    );
}

#[tracing::instrument]
pub fn spawn_node_task<S>(options: NodeTaskOptions<S>)
where
    S: TaskStateHolder,
{
    let task = options.task;
    let entry_path = task.plugin_path.join(&options.entrypoint);
    let entry_path = entry_path.to_string_lossy();

    #[cfg(windows)]
    let exe_path: PathBuf = options.runtime_path.join("node.exe");
    #[cfg(not(windows))]
    let exe_path: PathBuf = options.runtime_path.join("node");

    spawn_child_task(
        exe_path,
        task.plugin_path,
        task.logger,
        [
            entry_path.as_str(),
            "--connect-url",
            task.connect_url.as_str(),
            "--plugin-id",
            task.plugin_id.0.as_str(),
        ],
        task.state_handler,
    );
}

/// Spawns a child process that represents a plugin task being
/// executed by the server
#[tracing::instrument(name = "spawn_child_task", skip(task_state))]
pub fn spawn_child_task<I, S>(
    exe_path: PathBuf,
    working_dir: PathBuf,
    logger: TaskLogger,
    args: I,
    task_state: impl TaskStateHolder,
) -> Arc<Mutex<PluginTaskState>>
where
    I: IntoIterator<Item = S> + Debug,
    S: AsRef<OsStr>,
{
    let state = Arc::new(Mutex::new(PluginTaskState::NotStarted));

    // Exe does not exist
    if !exe_path.exists() {
        task_state.on_change_state(PluginTaskState::Unavailable);
        return state;
    }

    // Starting the exe
    task_state.on_change_state(PluginTaskState::Starting);

    // On unix platforms the file must be made executable
    #[cfg(unix)]
    if let Err(error) = crate::utils::file::make_file_executable(&exe_path) {
        tracing::error!(?error, "failed to make plugin executable");
        task_state.on_change_state(PluginTaskState::Error);
        return state;
    }

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
            task_state.on_change_state(PluginTaskState::Error);
            return state;
        }
    };

    let (task, handle) = ChildTask::create(child, logger);

    // Entered running state
    task_state.on_change_state(PluginTaskState::Running { handle });

    tokio::spawn(async move {
        let status = match task.run().await {
            Ok(child) => child,
            Err(cause) => {
                tracing::error!(?cause, "failed to get plugin output");
                task_state.on_change_state(PluginTaskState::Error);
                return;
            }
        };

        if status.success() {
            task_state.on_change_state(PluginTaskState::Stopped);
        } else {
            task_state.on_change_state(PluginTaskState::Error);
        }
    });

    state
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
    pub fn create(mut child: Child, logger: TaskLogger) -> (ChildTask, ChildTaskHandle) {
        // Take the error and output pipes
        let stdout = child.stdout.take().map(|io| BufReader::new(io).lines());
        let stderr = child.stderr.take().map(|io| BufReader::new(io).lines());

        tokio::spawn(
            async move {
                let _guard = logger.guard;

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
            .with_subscriber(logger.subscriber),
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
