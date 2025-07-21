import { createQuery } from "@tanstack/svelte-query";

import type { FolderId } from "../types/folders";
import type { ProfileId } from "../types/profiles";

import { foldersKeys } from "./folders.keys";
import { runeStore } from "../utils/svelte.svelte";
import { getFolder, getFolders } from "./folders.requests";

export function createFoldersQuery(profileId: () => ProfileId) {
  return createQuery(
    runeStore(() => {
      const id = profileId();
      return {
        queryKey: foldersKeys.list(id),
        queryFn: () => getFolders(id),
      };
    }),
  );
}

export function createFolderQuery(
  profileId: () => ProfileId,
  folderId: () => FolderId | null,
) {
  return createQuery(
    runeStore(() => {
      const pid = profileId();
      const fid = folderId();

      return {
        enabled: fid !== null,
        queryKey: foldersKeys.specific(pid, fid),
        queryFn: () => getFolder(fid!),
      };
    }),
  );
}
