use anyhow::Context;
use tauri::{AppHandle, Manager, State};

use crate::{
    commands::CmdResult,
    database::{
        entity::{
            folder::FolderId,
            tile::{CreateTile, TileIcon, TileId, TileModel, UpdateTile},
        },
        DbPool,
    },
    device::Devices,
};

/// Get a list of all tiles for a folder
#[tauri::command]
pub async fn tiles_get_tiles(
    db: State<'_, DbPool>,
    folder_id: FolderId,
) -> CmdResult<Vec<TileModel>> {
    let tiles = TileModel::get_by_folder(db.inner(), folder_id).await?;
    Ok(tiles)
}

/// Get a specific tile
#[tauri::command]
pub async fn tiles_get_tile(
    db: State<'_, DbPool>,
    tile_id: TileId,
) -> CmdResult<Option<TileModel>> {
    let tiles = TileModel::get_by_id(db.inner(), tile_id).await?;
    Ok(tiles)
}

/// Create a new tile
#[tauri::command]
pub async fn tiles_create_tile(
    db: State<'_, DbPool>,
    devices: State<'_, Devices>,
    create: CreateTile,
) -> CmdResult<TileModel> {
    let tile = TileModel::create(db.inner(), create).await?;

    tokio::spawn({
        let folder_id = tile.folder_id;
        let devices = devices.inner().clone();

        async move {
            let devices = devices;
            _ = devices.update_devices_tiles(folder_id).await;
        }
    });

    Ok(tile)
}

/// Update a specific tile
#[tauri::command]
pub async fn tiles_update_tile(
    app: AppHandle,
    db: State<'_, DbPool>,
    devices: State<'_, Devices>,

    tile_id: TileId,
    update: UpdateTile,
) -> CmdResult<TileModel> {
    let db = db.inner();
    let tile = TileModel::get_by_id(db, tile_id)
        .await?
        .context("tile not found")?;

    let previous_icon = &tile.config.icon;

    // Handle change in icon when using an uploaded icon (Remove the old file)
    if update
        .config
        .as_ref()
        .is_some_and(|config| previous_icon.ne(&config.icon))
    {
        if let TileIcon::Uploaded { path } = previous_icon {
            let app_data_path = app
                .path()
                .app_data_dir()
                .context("failed to get app data dir")?;
            let uploaded_icons = app_data_path.join("uploaded_icons");
            let file_path = uploaded_icons.join(path);
            if file_path.exists() {
                tokio::fs::remove_file(file_path).await?;
            }
        }
    }

    let tile = tile.update(db, update).await?;

    tokio::spawn({
        let folder_id = tile.folder_id;
        let devices = devices.inner().clone();

        async move {
            let devices = devices;
            _ = devices.update_devices_tiles(folder_id).await;
        }
    });

    Ok(tile)
}

/// Delete a specific tile
#[tauri::command]
pub async fn tiles_delete_tile(
    db: State<'_, DbPool>,
    devices: State<'_, Devices>,
    tile_id: TileId,
) -> CmdResult<()> {
    let db = db.inner();
    let tile = TileModel::get_by_id(db, tile_id)
        .await?
        .context("tile not found")?;

    TileModel::delete(db, tile_id).await?;

    tokio::spawn({
        let folder_id = tile.folder_id;
        let devices = devices.inner().clone();

        async move {
            let devices = devices;
            _ = devices.update_devices_tiles(folder_id).await;
        }
    });

    Ok(())
}
