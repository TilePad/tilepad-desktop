import type { QueryClient } from "@tanstack/svelte-query";

import type { ProfileId } from "../types/profiles";
import type { FolderId, FolderModel } from "../types/folders";

import { foldersKeys } from "./folders.keys";

export function invalidateFoldersList(
  client: QueryClient,
  profileId: ProfileId,
) {
  client.invalidateQueries({
    queryKey: foldersKeys.list(profileId),
    exact: false,
  });
}

export function updateFolderRows(
  client: QueryClient,
  profileId: ProfileId,
  folderId: FolderId,
  rows: number,
) {
  client.setQueryData<FolderModel>(
    foldersKeys.specific(profileId, folderId),
    (data) => {
      if (data === undefined) return data;
      return { ...data, config: { ...data.config, rows } };
    },
  );
}

export function updateFolderColumns(
  client: QueryClient,
  profileId: ProfileId,
  folderId: FolderId,
  columns: number,
) {
  client.setQueryData<FolderModel>(
    foldersKeys.specific(profileId, folderId),
    (data) => {
      if (data === undefined) return data;
      return { ...data, config: { ...data.config, columns } };
    },
  );
}
