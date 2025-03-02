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

export async function createTile(create: CreateTile) {
  const tile = await invoke<TileModel>("tiles_create_tile", {
    create,
  });

  invalidateTilesList(tile.folder_id);
  queryClient.setQueryData(tilesKeys.specific(tile.folder_id, tile.id), tile);

  return tile;
}

export async function updateTile(tileId: TileId, update: UpdateTile) {
  const tile = await invoke<TileModel>("tiles_update_tile", {
    tileId,
    update,
  });

  invalidateTilesList(tile.folder_id);
  queryClient.setQueryData(tilesKeys.specific(tile.folder_id, tile.id), tile);

  return tile;
}

export async function deleteTile(folderId: FolderId, tileId: TileId) {
  await invoke("tiles_delete_tile", { tileId });

  invalidateTilesList(folderId);
  queryClient.removeQueries({
    queryKey: tilesKeys.specific(folderId, tileId),
  });
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
  });
}

// [MUTATORS] ------------------------------------------------------

function invalidateTilesList(folderId: FolderId) {
  queryClient.invalidateQueries({
    queryKey: tilesKeys.list(folderId),
    exact: false,
  });
}
