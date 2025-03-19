use std::error::Error;

use anyhow::Context;
use device::Devices;
use plugin::PluginRegistry;
use tauri::{
    async_runtime::{block_on, spawn},
    App, Manager,
};
use tokio::sync::mpsc;
use tracing_subscriber::EnvFilter;

mod commands;
mod database;
mod device;
mod events;
#[allow(unused)]
mod plugin;
mod server;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use commands::{actions, devices, folders, plugins, profiles, server, tiles};

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            devices::devices_get_requests,
            devices::devices_get_devices,
            devices::devices_get_connected_devices,
            devices::devices_approve_request,
            devices::devices_decline_request,
            devices::devices_revoke_device,
            server::server_get_connection_info,
            profiles::profiles_get_profiles,
            profiles::profiles_get_profile,
            profiles::profiles_delete_profile,
            profiles::profiles_update_profile,
            profiles::profiles_create_profile,
            folders::folders_get_folders,
            folders::folders_get_folder,
            folders::folders_delete_folder,
            folders::folders_update_folder,
            folders::folders_create_folder,
            actions::actions_get_actions,
            actions::actions_get_action,
            tiles::tiles_get_tiles,
            tiles::tiles_get_tile,
            tiles::tiles_create_tile,
            tiles::tiles_update_tile,
            tiles::tiles_delete_tile,
            plugins::plugins_send_plugin_message,
            plugins::plugins_open_inspector,
            plugins::plugins_close_inspector,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let filter = EnvFilter::from_default_env();
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_env_filter(filter)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;

    let app_handle = app.handle();
    let app_data_path = app
        .path()
        .app_data_dir()
        .context("failed to get app data dir")?;

    let resources = app
        .path()
        .resource_dir()
        .context("failed to get resources directory")?;

    let core_resources = resources.join("core");
    let core_plugins = core_resources.join("plugins");
    let user_plugins = app_data_path.join("plugins");

    let db = block_on(database::connect_database(app_data_path.join("app.db")))
        .context("failed to load database")?;

    let (app_event_tx, app_event_rx) = mpsc::unbounded_channel();
    let plugins = PluginRegistry::new(app_event_tx.clone());
    let devices = Devices::new(app_event_tx.clone(), db.clone(), plugins.clone());

    app.manage(app_event_tx.clone());
    app.manage(db.clone());
    app.manage(devices.clone());
    app.manage(plugins.clone());

    tracing::debug!("starting event processor");

    // Load the core plugins into the registry
    spawn(plugin::load_plugins_into_registry(
        plugins.clone(),
        core_plugins,
    ));

    // Load the user plugins into the registry
    spawn(plugin::load_plugins_into_registry(
        plugins.clone(),
        user_plugins,
    ));

    // Spawn event processor
    spawn(events::processing::process_events(
        app_handle.clone(),
        db.clone(),
        app_event_rx,
    ));

    // Spawn HTTP server
    spawn(server::start_http_server(
        db,
        devices,
        app_handle.clone(),
        plugins.clone(),
    ));

    Ok(())
}
