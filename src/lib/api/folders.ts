import { invoke } from "@tauri-apps/api/core";
import { createQuery, createMutation } from "@tanstack/svelte-query";

import type { ProfileId } from "./types/profiles";
import type {
  FolderId,
  FolderModel,
  CreateFolder,
  UpdateFolder,
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

  invalidateFoldersList();
  queryClient.setQueryData(
    foldersKeys.specific(folder.profile_id, folder.id),
    folder,
  );

  return folder;
}

export async function updateFolder(folderId: FolderId, update: UpdateFolder) {
  const folder = await invoke<FolderModel>("folders_update_folder", {
    folderId,
    update,
  });

  invalidateFoldersList();
  queryClient.setQueryData(
    foldersKeys.specific(folder.profile_id, folder.id),
    folder,
  );

  return folder;
}

export async function deleteFolder(profileId: ProfileId) {
  await invoke("folders_delete_folder", { profileId });

  invalidateFoldersList();
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

function invalidateFoldersList() {
  queryClient.invalidateQueries({
    queryKey: foldersKeys.list,
    exact: false,
  });
}
