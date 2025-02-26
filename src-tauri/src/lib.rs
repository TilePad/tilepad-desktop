use std::error::Error;

use anyhow::Context;
use device::Devices;
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
mod plugin;
mod server;
mod services;
mod tile;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use commands::devices;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            devices::devices_get_requests,
            devices::devices_get_devices,
            devices::devices_get_connected_devices,
            devices::devices_approve_request,
            devices::devices_decline_request,
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

    let db = block_on(database::connect_database(app_data_path.join("app.db")))
        .context("failed to load database")?;

    let (app_event_tx, app_event_rx) = mpsc::unbounded_channel();
    let devices = Devices::new(app_event_tx, db.clone());

    app.manage(db.clone());
    app.manage(devices.clone());

    tracing::debug!("starting event processor");

    // Spawn event processor
    spawn(events::processing::process_events(
        app_handle.clone(),
        app_event_rx,
    ));

    // Spawn HTTP server
    spawn(server::start_http_server(db, devices, app_handle.clone()));

    Ok(())
}
