import { createQuery } from "@tanstack/svelte-query";

import type { TileId } from "../types/tiles";
import type { FolderId } from "../types/folders";

import { tilesKeys } from "./tiles.keys";
import { getTile, getTiles } from "./tiles.requests";

export function createTilesQuery(folderId: () => FolderId) {
  return createQuery(() => {
    const id = folderId();
    return {
      queryKey: tilesKeys.list(id),
      queryFn: () => getTiles(id!),
    };
  });
}

export function createTileQuery(
  folderId: () => FolderId,
  tileId: () => TileId,
) {
  return createQuery(() => {
    const fid = folderId();
    const tid = tileId();
    return {
      queryKey: tilesKeys.specific(fid, tid),
      queryFn: () => getTile(tid!),
    };
  });
}
