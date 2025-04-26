use std::{error::Error, str::FromStr, sync::Arc};

use anyhow::Context;
use device::Devices;
use events::DeepLinkContext;
use icons::Icons;
use plugin::Plugins;
use server::{HTTP_PORT, create_http_socket};
use std::path::PathBuf;
use tauri::{
    App, Manager,
    async_runtime::{block_on, spawn},
};
use tauri_plugin_deep_link::{DeepLinkExt, OpenUrlEvent};
use tile::Tiles;
use tilepad_manifest::plugin::PluginId;
use tokio::sync::mpsc;
use tracing_subscriber::EnvFilter;

mod commands;
mod database;
mod device;
mod events;
mod icons;
mod plugin;
mod server;
mod tile;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use commands::{actions, devices, folders, icons, plugins, profiles, server, tiles};

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| {}))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_opener::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            devices::devices_get_requests,
            devices::devices_get_devices,
            devices::devices_get_connected_devices,
            devices::devices_approve_request,
            devices::devices_decline_request,
            devices::devices_revoke_device,
            devices::devices_set_device_profile,
            devices::devices_set_device_folder,
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
            tiles::tiles_update_tile_properties,
            tiles::tiles_update_tile_label,
            tiles::tiles_update_tile_icon,
            tiles::tiles_update_tile_icon_options,
            tiles::tiles_delete_tile,
            plugins::plugins_send_plugin_message,
            plugins::plugins_open_inspector,
            plugins::plugins_close_inspector,
            plugins::plugins_get_plugin_properties,
            plugins::plugins_set_plugin_properties,
            plugins::plugins_get_plugins,
            plugins::plugins_stop_plugin_task,
            plugins::plugins_start_plugin_task,
            plugins::plugins_restart_plugin_task,
            plugins::plugins_reload_plugin,
            plugins::plugins_install_plugin_manual,
            plugins::plugins_uninstall_plugin,
            plugins::plugins_parse_manifest,
            plugins::plugins_download_bundle,
            icons::icons_get_icon_packs,
            icons::icons_install_icon_pack,
            icons::icons_uninstall_icon_pack,
            icons::icons_upload_user_icon,
            icons::icons_download_bundle,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    #[cfg(any(target_os = "linux", all(debug_assertions, windows)))]
    {
        use tauri_plugin_deep_link::DeepLinkExt;
        app.deep_link().register_all()?;
    }

    let filter = EnvFilter::from_default_env();
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_env_filter(filter)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;

    let app_handle = app.handle();
    let app_data_path = app
        .path()
        .app_data_dir()
        .context("failed to get app data dir")?;

    #[cfg(not(debug_assertions))]
    let core_resources = {
        let resources = app
            .path()
            .resource_dir()
            .context("failed to get resources directory")?;

        resources.join("core")
    };

    #[cfg(debug_assertions)]
    let core_resources = debug_core_resources_path();

    let core_plugins = core_resources.join("plugins");
    let user_plugins = app_data_path.join("plugins");
    let runtimes_path = app_data_path.join("runtimes");

    let user_icons = app_data_path.join("icons");
    let uploaded_icons = app_data_path.join("uploaded_icons");

    let db = block_on(database::connect_database(app_data_path.join("app.db")))
        .context("failed to load database")?;

    let (app_event_tx, app_event_rx) = mpsc::unbounded_channel();
    let icons = Arc::new(Icons::new(app_event_tx.clone(), user_icons, uploaded_icons));
    let plugins = Arc::new(Plugins::new(
        app_event_tx.clone(),
        db.clone(),
        core_plugins,
        user_plugins,
        runtimes_path,
    ));
    let devices = Arc::new(Devices::new(
        app_event_tx.clone(),
        db.clone(),
        plugins.clone(),
    ));
    let tiles = Arc::new(Tiles::new(db.clone(), icons.clone(), devices.clone()));

    app.manage(app_event_tx.clone());
    app.manage(db.clone());
    app.manage(devices.clone());
    app.manage(plugins.clone());
    app.manage(icons.clone());
    app.manage(tiles.clone());

    // Handle deep links (tilepad://deep-link/com.tilepad.system.system.tilePlugin#code=1)
    app.deep_link().on_open_url({
        tracing::debug!("prepared to handle deep links");

        let plugins = plugins.clone();
        move |event| on_deep_link(&plugins, event)
    });

    tracing::debug!("starting event processor");

    // Spawn event processor
    spawn(events::processing::process_events(
        app_handle.clone(),
        db.clone(),
        app_event_rx,
    ));

    // Binding a socket must come before the rest of the app setup
    // (Socket must be bound before plugins load to prevent phantom processes holding the port)
    let http_socket = match tauri::async_runtime::block_on(create_http_socket()) {
        Ok(value) => value,
        Err(cause) => {
            tracing::error!(?cause, "failed to bind http server socket");

            // Show error dialog about the failed port binding
            rfd::MessageDialog::new()
                .set_title("Failed to start")
                .set_description(format!(
                    "The port {} required to run TilePad is currently in use",
                    HTTP_PORT
                ))
                .set_level(rfd::MessageLevel::Error)
                .set_buttons(rfd::MessageButtons::Ok)
                .show();

            // Failing to start the HTTP socket is a fatal error, app will not work without this
            std::process::exit(1);
        }
    };

    // Spawn HTTP server
    spawn(server::start_http_server(
        http_socket,
        db,
        devices,
        plugins.clone(),
        icons.clone(),
        tiles.clone(),
    ));

    // Load the plugins from the default paths
    spawn({
        let plugins = plugins.clone();
        async move {
            plugins.load_defaults().await;
        }
    });

    // Load icon packs from the default paths
    spawn({
        let icons = icons.clone();
        async move {
            icons.load_defaults().await;
        }
    });

    Ok(())
}

fn on_deep_link(plugins: &Arc<Plugins>, event: OpenUrlEvent) {
    for url in event.urls() {
        tracing::debug!(?url, "execute deep link url");

        // Domain part must be "deep-link" to be treated as a deep link
        match url.domain() {
            Some("deep-link") => {}
            _ => continue,
        }

        let mut path = match url.path_segments() {
            Some(value) => value,
            None => continue,
        };

        let plugin_id = match path.next() {
            Some(value) => PluginId::from_str(value),
            None => continue,
        };

        let plugin_id = match plugin_id {
            Ok(value) => value,
            Err(cause) => {
                tracing::error!(?cause, "invalid deep link plugin id");
                continue;
            }
        };

        let host = url.host_str().map(|value| value.to_string());
        let path = url.path().to_string();
        let query = url.query().map(|value| value.to_string());
        let fragment = url.fragment().map(|value| value.to_string());
        let url = url.to_string();

        plugins.deep_link(
            &plugin_id,
            DeepLinkContext {
                url,
                host,
                path,
                query,
                fragment,
            },
        );
    }
}

/// In development we directly use the source path to "core" resources  
/// because tauri does not automatically update these files unless you
/// touch some source code in the main project and cause a re-build
#[cfg(debug_assertions)]
fn debug_core_resources_path() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable missing");

    let manifest_dir = std::path::Path::new(&manifest_dir);
    manifest_dir.join("../core")
}
