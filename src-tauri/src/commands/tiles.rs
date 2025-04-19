use std::sync::Arc;

use anyhow::Context;
use tauri::State;

use crate::{
    commands::CmdResult,
    database::{
        DbPool, JsonObject,
        entity::{
            folder::FolderId,
            tile::{CreateTile, TileIcon, TileId, TileLabel, TileModel, UpdateKind},
        },
    },
    device::Devices,
    icons::Icons,
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
    devices: State<'_, Arc<Devices>>,
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

/// Update a specific tile properties
#[tauri::command]
pub async fn tiles_update_tile_properties(
    db: State<'_, DbPool>,
    devices: State<'_, Arc<Devices>>,
    tile_id: TileId,
    properties: JsonObject,
    partial: bool,
) -> CmdResult<TileModel> {
    let db = db.inner();
    let tile = TileModel::get_by_id(db, tile_id)
        .await?
        .context("tile not found")?;
    let tile = tile.update_properties(db, properties, partial).await?;

    devices.background_update_folder(tile.folder_id);
    Ok(tile)
}

/// Update a specific tile label
#[tauri::command]
pub async fn tiles_update_tile_label(
    db: State<'_, DbPool>,
    devices: State<'_, Arc<Devices>>,
    tile_id: TileId,
    label: TileLabel,
    kind: UpdateKind,
) -> CmdResult<TileModel> {
    let db = db.inner();
    let tile = TileModel::get_by_id(db, tile_id)
        .await?
        .context("tile not found")?;
    let tile = tile.update_label(db, label, kind).await?;
    devices.background_update_folder(tile.folder_id);
    Ok(tile)
}

/// Update a specific tile label
#[tauri::command]
pub async fn tiles_update_tile_icon(
    db: State<'_, DbPool>,
    devices: State<'_, Arc<Devices>>,
    icons: State<'_, Arc<Icons>>,
    tile_id: TileId,
    icon: TileIcon,
    kind: UpdateKind,
) -> CmdResult<TileModel> {
    let db = db.inner();
    let tile = TileModel::get_by_id(db, tile_id)
        .await?
        .context("tile not found")?;

    // Handle change in icon when using an uploaded icon (Remove the old file)
    icons.handle_tile_change_icon(&tile.config.icon).await?;

    let tile = tile.update_icon(db, icon, kind).await?;
    devices.background_update_folder(tile.folder_id);
    Ok(tile)
}

/// Delete a specific tile
#[tauri::command]
pub async fn tiles_delete_tile(
    db: State<'_, DbPool>,
    devices: State<'_, Arc<Devices>>,
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
