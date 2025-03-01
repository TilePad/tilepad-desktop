use anyhow::Context;
use tauri::State;

use crate::{
    commands::CmdResult,
    database::{
        entity::{
            folder::FolderId,
            tile::{CreateTile, TileId, TileModel, UpdateTile},
        },
        DbPool,
    },
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
pub async fn tiles_create_tile(db: State<'_, DbPool>, create: CreateTile) -> CmdResult<TileModel> {
    let tile = TileModel::create(db.inner(), create).await?;
    Ok(tile)
}

/// Update a specific tile
#[tauri::command]
pub async fn tiles_update_tile(
    db: State<'_, DbPool>,
    tile_id: TileId,
    update: UpdateTile,
) -> CmdResult<TileModel> {
    let db = db.inner();
    let tile = TileModel::get_by_id(db, tile_id)
        .await?
        .context("tile not found")?;

    let tile = tile.update(db, update).await?;
    Ok(tile)
}

/// Delete a specific tile
#[tauri::command]
pub async fn tiles_delete_tile(db: State<'_, DbPool>, tile_id: TileId) -> CmdResult<()> {
    TileModel::delete(db.inner(), tile_id).await?;
    Ok(())
}
