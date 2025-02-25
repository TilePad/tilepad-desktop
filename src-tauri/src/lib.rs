use std::error::Error;

use anyhow::Context;
use device::Devices;
use tauri::{
    async_runtime::{block_on, spawn},
    App, Manager,
};

pub mod database;
pub mod device;
pub mod plugin;
pub mod server;
pub mod services;
pub mod tile;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let app_handle = app.handle();
    let app_data_path = app
        .path()
        .app_data_dir()
        .context("failed to get app data dir")?;

    let db = block_on(database::connect_database(app_data_path.join("app.db")))
        .context("failed to load database")?;

    let devices = Devices::default();

    app.manage(db.clone());
    app.manage(devices.clone());

    spawn(server::start_http_server(db, devices, app_handle.clone()));

    Ok(())
}
