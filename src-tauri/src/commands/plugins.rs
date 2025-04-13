use std::sync::Arc;

use crate::{
    commands::CmdResult,
    database::JsonObject,
    events::InspectorContext,
    plugin::{
        PluginWithState, Plugins,
        install::{install_plugin_requirements, install_plugin_zip, remove_plugin_files},
        loader::{load_plugin_from_path, read_plugin_manifest_zip},
        manifest::PluginId,
    },
};
use tilepad_manifest::plugin::Manifest as PluginManifest;

use anyhow::Context;
use tauri::State;

#[tauri::command]
pub async fn plugins_install_plugin_manual(
    plugins: State<'_, Arc<Plugins>>,
    data: Vec<u8>,
) -> CmdResult<()> {
    // Read the plugin manifest from within the zip file
    let manifest = read_plugin_manifest_zip(&data).await?;

    // Determine plugin install directory
    let plugin_id = &manifest.plugin.id;
    let path = plugins.user_path().join(&plugin_id.0);

    // Unload the plugin
    plugins.unload_plugin(plugin_id).await;

    // Cleanup old files
    remove_plugin_files(&path).await?;

    // Install the plugin zip file
    install_plugin_zip(&data, &path).await?;

    // Install plugin runtime if one is required
    install_plugin_requirements(&manifest, &plugins.runtimes_path()).await?;

    // Load the plugin
    let plugin = load_plugin_from_path(&path)
        .await
        .context("failed to load plugin")?;

    plugins.load_plugin(plugin).await;

    Ok(())
}

#[tauri::command]
pub async fn plugins_uninstall_plugin(
    plugins: State<'_, Arc<Plugins>>,
    plugin_id: PluginId,
) -> CmdResult<()> {
    // Determine plugin install directory
    let path = plugins.user_path().join(&plugin_id.0);

    // Unload the plugin
    plugins.unload_plugin(&plugin_id).await;

    // Cleanup old files
    remove_plugin_files(&path).await?;

    Ok(())
}

#[tauri::command]
pub async fn plugins_send_plugin_message(
    plugins: State<'_, Arc<Plugins>>,
    context: InspectorContext,
    message: serde_json::Value,
) -> CmdResult<()> {
    plugins.handle_send_message(context, message).await?;

    Ok(())
}

#[tauri::command]
pub async fn plugins_open_inspector(
    plugins: State<'_, Arc<Plugins>>,
    context: InspectorContext,
) -> CmdResult<()> {
    plugins.open_inspector(context);
    Ok(())
}

#[tauri::command]
pub async fn plugins_close_inspector(
    plugins: State<'_, Arc<Plugins>>,
    context: InspectorContext,
) -> CmdResult<()> {
    plugins.close_inspector(context);
    Ok(())
}

#[tauri::command]
pub async fn plugins_get_plugin_properties(
    plugins: State<'_, Arc<Plugins>>,
    plugin_id: PluginId,
) -> CmdResult<JsonObject> {
    let result = plugins.get_plugin_properties(plugin_id).await?;
    Ok(result)
}

#[tauri::command]
pub async fn plugins_set_plugin_properties(
    plugins: State<'_, Arc<Plugins>>,
    plugin_id: PluginId,
    properties: JsonObject,
    partial: bool,
) -> CmdResult<()> {
    plugins
        .set_plugin_properties(plugin_id, properties, partial)
        .await?;
    Ok(())
}

#[tauri::command]
pub fn plugins_get_plugins(plugins: State<'_, Arc<Plugins>>) -> Vec<PluginWithState> {
    plugins.get_plugins_with_state()
}

#[tauri::command]
pub async fn plugins_stop_plugin_task(
    plugins: State<'_, Arc<Plugins>>,
    plugin_id: PluginId,
) -> CmdResult<()> {
    plugins.stop_task(&plugin_id).await;
    Ok(())
}

#[tauri::command]
pub async fn plugins_start_plugin_task(
    plugins: State<'_, Arc<Plugins>>,
    plugin_id: PluginId,
) -> CmdResult<()> {
    let plugin = plugins.get_plugin(&plugin_id).context("plugin not found")?;

    plugins
        .start_task(plugin_id, plugin.path.clone(), &plugin.manifest)
        .await;

    Ok(())
}

#[tauri::command]
pub async fn plugins_restart_plugin_task(
    plugins: State<'_, Arc<Plugins>>,
    plugin_id: PluginId,
) -> CmdResult<()> {
    let plugin = plugins.get_plugin(&plugin_id).context("plugin not found")?;

    plugins
        .restart_task(plugin_id, plugin.path.clone(), &plugin.manifest)
        .await;

    Ok(())
}

#[tauri::command]
pub async fn plugins_reload_plugin(
    plugins: State<'_, Arc<Plugins>>,
    plugin_id: PluginId,
) -> CmdResult<()> {
    // Unload the plugin
    let plugin = plugins
        .unload_plugin(&plugin_id)
        .await
        .context("plugin was never loaded")?;

    // Load the new plugin from the same path
    let new_plugin = load_plugin_from_path(&plugin.path).await?;

    // Load the new plugin into the plugins registry
    plugins.load_plugin(new_plugin).await;
    Ok(())
}

#[tauri::command]
pub fn plugins_parse_manifest(manifest: String) -> CmdResult<PluginManifest> {
    let manifest: PluginManifest = PluginManifest::parse(&manifest)?;
    Ok(manifest)
}
