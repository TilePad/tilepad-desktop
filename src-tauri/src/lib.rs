use std::{error::Error, str::FromStr, sync::Arc};

use anyhow::Context;
use database::{DbPool, entity::settings::SettingsModel};
use device::Devices;
use events::DeepLinkContext;
use fonts::Fonts;
use icons::Icons;
use plugin::Plugins;
use server::{HTTP_PORT, create_http_socket};
use std::path::PathBuf;
use tauri::{
    App, AppHandle, Manager, RunEvent,
    async_runtime::{block_on, spawn},
};
use tauri_plugin_deep_link::{DeepLinkExt, OpenUrlEvent};
use tile::Tiles;
use tilepad_manifest::plugin::PluginId;
use tokio::{fs::create_dir_all, sync::mpsc};
use utils::tracing::setup_main_subscriber;

mod commands;
mod database;
mod device;
mod events;
mod fonts;
mod icons;
mod plugin;
mod server;
mod tile;
mod tray;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use commands::{
        actions, devices, folders, fonts, icons, plugins, profiles, server, settings, tiles,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(
            handle_duplicate_instance,
        ))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_opener::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            // Devices
            devices::devices_get_requests,
            devices::devices_get_devices,
            devices::devices_get_connected_devices,
            devices::devices_approve_request,
            devices::devices_decline_request,
            devices::devices_revoke_device,
            devices::devices_set_device_profile,
            devices::devices_set_device_folder,
            // Server
            server::server_get_connection_info,
            server::server_get_licenses,
            // Profiles
            profiles::profiles_get_profiles,
            profiles::profiles_get_profile,
            profiles::profiles_delete_profile,
            profiles::profiles_set_name,
            profiles::profiles_create_profile,
            // Folders
            folders::folders_get_folders,
            folders::folders_get_folder,
            folders::folders_delete_folder,
            folders::folders_set_name,
            folders::folders_set_config,
            folders::folders_create_folder,
            // Actions
            actions::actions_get_actions,
            actions::actions_get_action,
            // Tiles
            tiles::tiles_get_tiles,
            tiles::tiles_get_tile,
            tiles::tiles_create_tile,
            tiles::tiles_update_tile_properties,
            tiles::tiles_update_tile_label,
            tiles::tiles_update_tile_icon,
            tiles::tiles_update_tile_icon_options,
            tiles::tiles_delete_tile,
            tiles::tiles_update_tile_position,
            // Plugins
            plugins::plugins_send_plugin_message,
            plugins::plugins_send_plugin_display_message,
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
            // Icons
            icons::icons_get_icon_packs,
            icons::icons_install_icon_pack,
            icons::icons_uninstall_icon_pack,
            icons::icons_upload_user_icon,
            icons::icons_download_bundle,
            // Fonts
            fonts::fonts_fonts,
            // Settings
            settings::settings_get_settings,
            settings::settings_set_settings
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        // Prevent default exit handling, app exiting is done
        .run(handle_app_event);
}

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    // For debug mode we load the deep link at runtime to provide
    // deep linking while developing
    #[cfg(any(target_os = "linux", all(debug_assertions, windows)))]
    {
        use tauri_plugin_deep_link::DeepLinkExt;
        app.deep_link().register_all()?;
    }

    let app_handle = app.handle();
    let app_data_path = app
        .path()
        .app_data_dir()
        .context("failed to get app data dir")?;

    let logs_path = app
        .path()
        .app_log_dir()
        .context("failed to get logging path")?;

    // Create the logs directory
    if !logs_path.exists() {
        _ = block_on(create_dir_all(&logs_path));
    }

    // Setup tracing
    let worker_guard = setup_main_subscriber(logs_path.clone())?;

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

    let db = match block_on(database::connect_database(app_data_path.join("app.db"))) {
        Ok(value) => value,
        Err(cause) => {
            tracing::error!(?cause, "failed to load database");
            std::process::exit(1);
        }
    };

    // Load database settings
    let settings = block_on(SettingsModel::get_or_default(&db))
        .map(|value| value.config)
        .unwrap_or_default();

    // Setup automatic startup
    setup_auto_start(app_handle, settings.start_automatically);

    // Handle minimized startup for autostart
    if settings.start_minimized && is_auto_started() {
        close_app_window(app_handle);
    }

    let (app_event_tx, app_event_rx) = mpsc::unbounded_channel();
    let icons = Arc::new(Icons::new(app_event_tx.clone(), user_icons, uploaded_icons));
    let plugins = Arc::new(Plugins::new(
        app_event_tx.clone(),
        db.clone(),
        core_plugins,
        user_plugins,
        runtimes_path,
        logs_path,
    ));
    let devices = Arc::new(Devices::new(
        app_event_tx.clone(),
        db.clone(),
        plugins.clone(),
    ));
    let tiles = Arc::new(Tiles::new(db.clone(), icons.clone(), devices.clone()));
    let fonts = Arc::new(Fonts::new());

    app.manage(app_event_tx.clone());
    app.manage(db.clone());
    app.manage(devices.clone());
    app.manage(plugins.clone());
    app.manage(icons.clone());
    app.manage(tiles.clone());
    app.manage(fonts.clone());
    app.manage(worker_guard);

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
        fonts,
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

    tray::create_tray_menu(app)?;

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

/// Handle initialization of a second app instance, focuses the main
/// window instead of allowing multiple instances
fn handle_duplicate_instance(app: &AppHandle, _args: Vec<String>, _cwd: String) {
    let _ = app
        .get_webview_window("main")
        .expect("no main window")
        .set_focus();
}

/// Closes the main app window
fn close_app_window(app: &AppHandle) {
    let _ = app
        .get_webview_window("main")
        .expect("no main window")
        .close();
}

/// Handles app events, used for the minimize to tray event
fn handle_app_event(app: &AppHandle, event: RunEvent) {
    if let tauri::RunEvent::ExitRequested { api, code, .. } = event {
        let db = app.state::<DbPool>();
        let settings = block_on(SettingsModel::get_or_default(db.inner()));
        let minimize_to_tray = settings.is_ok_and(|value| value.config.minimize_tray);

        if code.is_none() && minimize_to_tray {
            api.prevent_exit();
        }
    }
}

/// Handles app events, used for the minimize to tray event
fn setup_auto_start(app: &AppHandle, auto_start: bool) {
    use tauri_plugin_autostart::MacosLauncher;
    use tauri_plugin_autostart::ManagerExt;

    _ = app.app_handle().plugin(tauri_plugin_autostart::init(
        MacosLauncher::LaunchAgent,
        Some(vec!["--auto-start"]),
    ));

    // Get the autostart manager
    let autostart_manager = app.autolaunch();

    if auto_start {
        // Enable autostart
        let _ = autostart_manager.enable();
    } else {
        // Disable autostart
        let _ = autostart_manager.disable();
    }
}

fn is_auto_started() -> bool {
    std::env::args().any(|arg| arg == "--auto-start")
}
