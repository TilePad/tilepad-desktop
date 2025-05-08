//! # Tray
//!
//! Tray menu logic for the minimize to tray functionality

use anyhow::Context;
use tauri::{
    App, AppHandle, Manager,
    menu::{IconMenuItem, Menu, MenuItem},
    tray::TrayIconBuilder,
};

#[cfg(not(debug_assertions))]
const TRAY_NAME: &str = "TilePad";
#[cfg(debug_assertions)]
const TRAY_NAME: &str = "TilePad (Dev)";

/// Creates a tray menu for the app
pub fn create_tray_menu(app: &mut App) -> anyhow::Result<()> {
    let icon = app
        .default_window_icon()
        .context("failed to get app icon")?
        .clone();

    let title_i = IconMenuItem::new(app, TRAY_NAME, false, Some(icon.clone()), None::<&str>)?;
    let open_i = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&title_i, &open_i, &quit_i])?;

    TrayIconBuilder::new()
        .icon(icon)
        .tooltip("TilePad")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(move |app, event| {
            if event.id() == open_i.id() {
                handle_open_clicked(app).expect("failed to open")
            } else if event.id == quit_i.id() {
                handle_quit_clicked(app).expect("failed to quit")
            }
        })
        .build(app)?;

    Ok(())
}

/// Handles the "Open" button in the tray menu being clicked
/// to bring the app back into focus or re-create the window
/// if it has been closed
fn handle_open_clicked(app: &AppHandle) -> anyhow::Result<()> {
    if let Some((_name, window)) = app.webview_windows().iter().next() {
        // Show existing window if none are present
        window.show().context("failed to show window")?;

        // Focus the window
        window.set_focus().context("failed to focus window")?;
    } else {
        // Recreate the web view from the existing config
        recreate_window(app).context("failed to recreate window")?;
    }

    Ok(())
}

/// Handles "Quit" being clicked in the tray menu, closes the
/// application
fn handle_quit_clicked(app: &AppHandle) -> anyhow::Result<()> {
    app.exit(0);
    Ok(())
}

/// Re-creates the main application window from the
/// existing configuration.
///
/// Used to bring the window back using the tray menu
/// when it's been closed.
fn recreate_window(app: &AppHandle) -> anyhow::Result<()> {
    _ = tauri::WebviewWindowBuilder::from_config(
        app,
        &app.config()
            .app
            .windows
            .first()
            .context("missing first window config")?
            .clone(),
    )
    .context("failed to recreate window from config")?
    .build()
    .context("failed to create new window")?;

    Ok(())
}
