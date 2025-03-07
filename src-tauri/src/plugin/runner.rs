use std::path::Path;

use tokio::process::{Child, Command};

/// Plugin task that is handling running the plugin itself
#[allow(unused)]
pub enum PluginTask {
    /// Node child process is running the task
    Node(Child),
    /// Native child process is running the task
    Native(Child),
}

#[allow(unused)]
pub async fn spawn_node_plugin_task(
    node_path: &Path,
    script_path: &Path,
    plugin_path: &Path,

    server_port: u16,
    plugin_uuid: String,
) -> anyhow::Result<PluginTask> {
    let node_exe = node_path.join("node.exe");

    let child = Command::new(node_exe)
        .current_dir(plugin_path)
        // Specify script to run
        .arg(script_path)
        // Specify port
        .arg("-port")
        .arg(server_port.to_string())
        // Specify plugin UUID
        .arg("-pluginUUID")
        .arg(plugin_uuid)
        // Specify register event
        .arg("-registerEvent")
        .arg("registerPlugin")
        // TODO: Specify info
        .arg("-info")
        .arg("{}")
        .spawn()?;

    Ok(PluginTask::Node(child))
}

#[allow(unused)]
pub async fn spawn_native_plugin_task() -> anyhow::Result<PluginTask> {
    todo!()
}
