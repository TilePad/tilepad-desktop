import { invoke } from "@tauri-apps/api/core";

import type { FolderId } from "../types/folders";
import type {
  TileId,
  TileIcon,
  TileModel,
  TileLabel,
  CreateTile,
  UpdateKind,
  TilePosition,
  TileIconOptions,
} from "../types/tiles";

export function getTiles(folderId: FolderId) {
  return invoke<TileModel[]>("tiles_get_tiles", { folderId });
}

export function getTile(tileId: TileId) {
  return invoke<TileModel | null>("tiles_get_tile", { tileId });
}

export function createTile(create: CreateTile) {
  return invoke<TileModel>("tiles_create_tile", {
    create,
  });
}

export function updateTilePosition(tileId: TileId, position: TilePosition) {
  return invoke<TileModel>("tiles_update_tile_position", {
    tileId,
    position,
  });
}

export function updateTileProperties(
  tileId: TileId,
  properties: object,
  partial: boolean,
) {
  return invoke<TileModel>("tiles_update_tile_properties", {
    tileId,
    properties,
    partial,
  });
}

export function updateTileLabel(
  tileId: TileId,
  label: TileLabel,
  kind: UpdateKind,
) {
  return invoke<TileModel>("tiles_update_tile_label", {
    tileId,
    label,
    kind,
  });
}

export function updateTileIcon(
  tileId: TileId,
  icon: TileIcon,
  kind: UpdateKind,
) {
  return invoke<TileModel>("tiles_update_tile_icon", {
    tileId,
    icon,
    kind,
  });
}

export function updateTileIconOptions(
  tileId: TileId,
  iconOptions: TileIconOptions,
) {
  return invoke<TileModel>("tiles_update_tile_icon_options", {
    tileId,
    iconOptions,
  });
}

export function deleteTile(tileId: TileId) {
  return invoke("tiles_delete_tile", { tileId });
}
