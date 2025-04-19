import { invoke } from "@tauri-apps/api/core";
import { createQuery, createMutation } from "@tanstack/svelte-query";

import type { FolderId } from "./types/folders";
import type {
  TileId,
  TileIcon,
  TileModel,
  TileLabel,
  CreateTile,
  UpdateKind,
} from "./types/tiles";

import { queryClient } from "./client";
import { runeStore } from "./utils/svelte.svelte";

export const tilesKeys = {
  root: ["tiles"],
  list: (folderId: FolderId | null) => ["tiles", folderId, "list"],
  specific: (folderId: FolderId | null, tileId: TileId | null) => [
    "tiles",
    folderId,
    "specific",
    tileId,
  ],
};

// [REQUESTS] ------------------------------------------------------

function getTiles(folderId: FolderId) {
  return invoke<TileModel[]>("tiles_get_tiles", { folderId });
}

export function getTile(tileId: TileId) {
  return invoke<TileModel | null>("tiles_get_tile", { tileId });
}

function createTile(create: CreateTile) {
  return invoke<TileModel>("tiles_create_tile", {
    create,
  });
}

function updateTileProperties(
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

function updateTileLabel(tileId: TileId, label: TileLabel, kind: UpdateKind) {
  return invoke<TileModel>("tiles_update_tile_label", {
    tileId,
    label,
    kind,
  });
}

function updateTileIcon(tileId: TileId, icon: TileIcon, kind: UpdateKind) {
  return invoke<TileModel>("tiles_update_tile_icon", {
    tileId,
    icon,
    kind,
  });
}

function deleteTile(tileId: TileId) {
  return invoke("tiles_delete_tile", { tileId });
}

// [QUERIES] ------------------------------------------------------

export function createTilesQuery(folderId: () => FolderId) {
  return createQuery(
    runeStore(() => {
      const id = folderId();
      return {
        queryKey: tilesKeys.list(id),
        queryFn: () => getTiles(id!),
      };
    }),
  );
}

export function createTileQuery(
  folderId: () => FolderId,
  tileId: () => TileId,
) {
  return createQuery(
    runeStore(() => {
      const fid = folderId();
      const tid = tileId();
      return {
        queryKey: tilesKeys.specific(fid, tid),
        queryFn: () => getTile(tid!),
      };
    }),
  );
}

// [MUTATIONS] ------------------------------------------------------

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

export function createDeleteTileMutation(folderId: FolderId) {
  return createMutation({
    mutationFn: ({ tileId }: { tileId: TileId }) => deleteTile(tileId),
    onSuccess: (_, { tileId }) => {
      invalidateTilesList(folderId);
      queryClient.removeQueries({
        queryKey: tilesKeys.specific(folderId, tileId),
      });
    },
  });
}

// [MUTATORS] ------------------------------------------------------

function invalidateTilesList(folderId: FolderId) {
  queryClient.invalidateQueries({
    queryKey: tilesKeys.list(folderId),
    exact: false,
  });
}
