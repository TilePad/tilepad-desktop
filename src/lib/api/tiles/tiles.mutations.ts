import { createMutation } from "@tanstack/svelte-query";

import type { FolderId } from "../types/folders";
import type {
  TileId,
  TileIcon,
  TileLabel,
  CreateTile,
  UpdateKind,
  TilePosition,
  TileIconOptions,
} from "../types/tiles";

import { queryClient } from "../client";
import { tilesKeys } from "./tiles.keys";
import { invalidateTilesList } from "./tiles.mutators";
import {
  createTile,
  deleteTile,
  updateTileIcon,
  updateTileLabel,
  updateTilePosition,
  updateTileProperties,
  updateTileIconOptions,
} from "./tiles.requests";

export function createCreateTileMutation() {
  return createMutation({
    mutationFn: ({ create }: { create: CreateTile }) => createTile(create),
    onSuccess: (tile) => {
      invalidateTilesList(tile.folder_id);
      queryClient.setQueryData(
        tilesKeys.specific(tile.folder_id, tile.id),
        tile,
      );
    },
  });
}

export function createUpdateTilePositionMutation() {
  return createMutation({
    scope: { id: "tile" },
    mutationFn: ({
      tileId,
      position,
    }: {
      tileId: TileId;
      position: TilePosition;
    }) => updateTilePosition(tileId, position),
    onSuccess: (tile) => {
      invalidateTilesList(tile.folder_id);
      queryClient.setQueryData(
        tilesKeys.specific(tile.folder_id, tile.id),
        tile,
      );
    },
  });
}

export function createUpdateTilePropertiesMutation() {
  return createMutation({
    scope: { id: "tile" },
    mutationFn: ({
      tileId,
      properties,
      partial,
    }: {
      tileId: TileId;
      properties: object;
      partial: boolean;
    }) => updateTileProperties(tileId, properties, partial),
    onSuccess: (tile) => {
      invalidateTilesList(tile.folder_id);
      queryClient.setQueryData(
        tilesKeys.specific(tile.folder_id, tile.id),
        tile,
      );
    },
  });
}

export function createUpdateTileLabelMutation() {
  return createMutation({
    scope: { id: "tile" },
    mutationFn: ({
      tileId,
      label,
      kind,
    }: {
      tileId: TileId;
      label: TileLabel;
      kind: UpdateKind;
    }) => updateTileLabel(tileId, label, kind),
    onSuccess: (tile) => {
      invalidateTilesList(tile.folder_id);
      queryClient.setQueryData(
        tilesKeys.specific(tile.folder_id, tile.id),
        tile,
      );
    },
  });
}

export function createUpdateTileIconMutation() {
  return createMutation({
    scope: { id: "tile" },
    mutationFn: ({
      tileId,
      icon,
      kind,
    }: {
      tileId: TileId;
      icon: TileIcon;
      kind: UpdateKind;
    }) => updateTileIcon(tileId, icon, kind),
    onSuccess: (tile) => {
      invalidateTilesList(tile.folder_id);
      queryClient.setQueryData(
        tilesKeys.specific(tile.folder_id, tile.id),
        tile,
      );
    },
  });
}

export function createUpdateTileIconOptionsMutation() {
  return createMutation({
    scope: { id: "tile" },
    mutationFn: ({
      tileId,
      iconOptions,
    }: {
      tileId: TileId;
      iconOptions: TileIconOptions;
    }) => updateTileIconOptions(tileId, iconOptions),
    onSuccess: (tile) => {
      invalidateTilesList(tile.folder_id);
      queryClient.setQueryData(
        tilesKeys.specific(tile.folder_id, tile.id),
        tile,
      );
    },
  });
}

export function createDeleteTileMutation() {
  return createMutation({
    mutationFn: ({ tileId }: { tileId: TileId; folderId: FolderId }) =>
      deleteTile(tileId),
    onSuccess: (_, { tileId, folderId }) => {
      invalidateTilesList(folderId);
      queryClient.removeQueries({
        queryKey: tilesKeys.specific(folderId, tileId),
      });
    },
  });
}
