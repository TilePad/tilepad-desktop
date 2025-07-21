import { invoke } from "@tauri-apps/api/core";

import type { ProfileId } from "../types/profiles";
import type {
  FolderId,
  FolderModel,
  CreateFolder,
  FolderConfig,
} from "../types/folders";

export function getFolders(profileId: ProfileId) {
  return invoke<FolderModel[]>("folders_get_folders", {
    profileId,
  });
}

export function getFolder(folderId: FolderId) {
  return invoke<FolderModel | null>("folders_get_folder", { folderId });
}

export function createFolder(create: CreateFolder) {
  return invoke<FolderModel>("folders_create_folder", {
    create,
  });
}

export function setFolderName(folderId: FolderId, name: string) {
  return invoke<FolderModel>("folders_set_name", {
    folderId,
    name,
  });
}

export function setFolderConfig(folderId: FolderId, config: FolderConfig) {
  return invoke<FolderModel>("folders_set_config", {
    folderId,
    config,
  });
}

export function deleteFolder(folderId: FolderId) {
  return invoke<object>("folders_delete_folder", { folderId });
}
