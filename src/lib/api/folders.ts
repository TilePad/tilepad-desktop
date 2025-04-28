import { invoke } from "@tauri-apps/api/core";
import { createQuery, createMutation } from "@tanstack/svelte-query";

import type { ProfileId } from "./types/profiles";
import type {
  FolderId,
  FolderModel,
  CreateFolder,
  FolderConfig,
} from "./types/folders";

import { queryClient } from "./client";
import { runeStore } from "./utils/svelte.svelte";

export const foldersKeys = {
  root: ["folders"],
  list: (profileId: ProfileId) => ["folders", "profile", profileId, "list"],

  specific: (profileId: ProfileId, folderId: FolderId | null) => [
    "folders",
    "profile",
    profileId,
    "folder",
    folderId,
  ],
};

// [REQUESTS] ------------------------------------------------------

function getFolders(profileId: ProfileId) {
  return invoke<FolderModel[]>("folders_get_folders", {
    profileId,
  });
}

function getFolder(folderId: FolderId) {
  return invoke<FolderModel | null>("folders_get_folder", { folderId });
}

export async function createFolder(create: CreateFolder) {
  const folder = await invoke<FolderModel>("folders_create_folder", {
    create,
  });

  invalidateFoldersList(folder.profile_id);
  queryClient.setQueryData(
    foldersKeys.specific(folder.profile_id, folder.id),
    folder,
  );

  return folder;
}

export async function setFolderName(folderId: FolderId, name: string) {
  const folder = await invoke<FolderModel>("folders_set_name", {
    folderId,
    name,
  });

  invalidateFoldersList(folder.profile_id);
  queryClient.setQueryData(
    foldersKeys.specific(folder.profile_id, folder.id),
    folder,
  );

  return folder;
}

export async function setFolderConfig(
  folderId: FolderId,
  config: FolderConfig,
) {
  const folder = await invoke<FolderModel>("folders_set_config", {
    folderId,
    config,
  });

  invalidateFoldersList(folder.profile_id);
  queryClient.setQueryData(
    foldersKeys.specific(folder.profile_id, folder.id),
    folder,
  );

  return folder;
}

export async function deleteFolder(profileId: ProfileId, folderId: FolderId) {
  await invoke("folders_delete_folder", { folderId });

  invalidateFoldersList(profileId);
}

// [QUERIES] ------------------------------------------------------

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

// [MUTATIONS] ------------------------------------------------------

export function createCreateFolderMutation() {
  return createMutation({
    mutationFn: ({ create }: { create: CreateFolder }) => createFolder(create),
  });
}

// [MUTATORS] ------------------------------------------------------

function invalidateFoldersList(profileId: ProfileId) {
  queryClient.invalidateQueries({
    queryKey: foldersKeys.list(profileId),
    exact: false,
  });
}
