import { invoke } from "@tauri-apps/api/core";

import type { ProfileId } from "../types/profiles";
import type {
  FolderId,
  FolderModel,
  CreateFolder,
  FolderConfig,
} from "../types/folders";

import { queryClient } from "../client";
import { foldersKeys } from "./folders.keys";
import { invalidateFoldersList } from "./folders.mutators";

export function getFolders(profileId: ProfileId) {
  return invoke<FolderModel[]>("folders_get_folders", {
    profileId,
  });
}

export function getFolder(folderId: FolderId) {
  return invoke<FolderModel | null>("folders_get_folder", { folderId });
}

export async function createFolder(create: CreateFolder) {
  const folder = await invoke<FolderModel>("folders_create_folder", {
    create,
  });

  invalidateFoldersList(queryClient, folder.profile_id);
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

  invalidateFoldersList(queryClient, folder.profile_id);
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

  invalidateFoldersList(queryClient, folder.profile_id);
  queryClient.setQueryData(
    foldersKeys.specific(folder.profile_id, folder.id),
    folder,
  );

  return folder;
}

export async function deleteFolder(profileId: ProfileId, folderId: FolderId) {
  await invoke("folders_delete_folder", { folderId });

  invalidateFoldersList(queryClient, profileId);
}
