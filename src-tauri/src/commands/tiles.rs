use std::sync::Arc;

use anyhow::Context;
use tauri::State;

use crate::{
    commands::CmdResult,
    database::{
        DbPool, JsonObject,
        entity::{
            folder::FolderId,
            tile::{
                CreateTile, TileIcon, TileIconOptions, TileId, TileLabel, TileModel, TilePosition,
                UpdateKind,
            },
        },
    },
    device::Devices,
    tile::Tiles,
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
    let db = db.inner();
    let tile = TileModel::create(db, create).await?;
    devices.background_update_folder(tile.folder_id);
    Ok(tile)
}

/// Update a specific tile properties
#[tauri::command]
pub async fn tiles_update_tile_properties(
    tiles: State<'_, Arc<Tiles>>,
    tile_id: TileId,
    properties: JsonObject,
    partial: bool,
) -> CmdResult<TileModel> {
    let tile = tiles
        .update_tile_properties(tile_id, None, properties, partial)
        .await?;
    Ok(tile)
}

/// Update a specific tile label
#[tauri::command]
pub async fn tiles_update_tile_label(
    tiles: State<'_, Arc<Tiles>>,
    tile_id: TileId,
    label: TileLabel,
    kind: UpdateKind,
) -> CmdResult<TileModel> {
    let tile = tiles.update_tile_label(tile_id, None, label, kind).await?;

    Ok(tile)
}
/// Update a specific tile label
#[tauri::command]
pub async fn tiles_update_tile_position(
    tiles: State<'_, Arc<Tiles>>,
    tile_id: TileId,
    position: TilePosition,
) -> CmdResult<TileModel> {
    let tile = tiles.update_tile_position(tile_id, None, position).await?;

    Ok(tile)
}

/// Update a specific tile icon
#[tauri::command]
pub async fn tiles_update_tile_icon(
    tiles: State<'_, Arc<Tiles>>,
    tile_id: TileId,
    icon: TileIcon,
    kind: UpdateKind,
) -> CmdResult<TileModel> {
    let tile = tiles.update_tile_icon(tile_id, None, icon, kind).await?;
    Ok(tile)
}

/// Update a specific tile icon options
#[tauri::command]
pub async fn tiles_update_tile_icon_options(
    tiles: State<'_, Arc<Tiles>>,
    tile_id: TileId,
    icon_options: TileIconOptions,
) -> CmdResult<TileModel> {
    let tile = tiles
        .update_tile_icon_options(tile_id, None, icon_options)
        .await?;
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
    devices.background_update_folder(tile.folder_id);
    Ok(())
}
