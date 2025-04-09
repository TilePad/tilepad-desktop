import { invoke } from "@tauri-apps/api/core";
import { createQuery, createMutation } from "@tanstack/svelte-query";

import type { FolderId } from "./types/folders";
import type { TileId, TileModel, CreateTile, UpdateTile } from "./types/tiles";

import { queryClient } from "./client";
import { runeStore } from "./utils/svelte.svelte";

const tilesKeys = {
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

function getTile(tileId: TileId) {
  return invoke<TileModel | null>("tiles_get_tile", { tileId });
}

function createTile(create: CreateTile) {
  return invoke<TileModel>("tiles_create_tile", {
    create,
  });
}

function updateTile(tileId: TileId, update: UpdateTile) {
  return invoke<TileModel>("tiles_update_tile", {
    tileId,
    update,
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

export function createUpdateTileMutation() {
  return createMutation({
    mutationFn: ({ tileId, update }: { tileId: TileId; update: UpdateTile }) =>
      updateTile(tileId, update),
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
