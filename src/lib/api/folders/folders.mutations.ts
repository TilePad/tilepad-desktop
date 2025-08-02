import { createMutation, getQueryClientContext } from "@tanstack/svelte-query";

import type { ProfileId } from "../types/profiles";
import type { FolderId, CreateFolder, FolderConfig } from "../types/folders";

import { foldersKeys } from "./folders.keys";
import { invalidateFoldersList } from "./folders.mutators";
import {
  createFolder,
  deleteFolder,
  setFolderName,
  setFolderConfig,
} from "./folders.requests";

export function createCreateFolderMutation() {
  const queryClient = getQueryClientContext();

  return createMutation(() => ({
    mutationFn: ({ create }: { create: CreateFolder }) => createFolder(create),
    onSuccess(folder) {
      invalidateFoldersList(queryClient, folder.profile_id);
      queryClient.setQueryData(
        foldersKeys.specific(folder.profile_id, folder.id),
        folder,
      );
    },
  }));
}

export function createSetFolderNameMutation() {
  const queryClient = getQueryClientContext();

  return createMutation(() => ({
    mutationFn: ({ folderId, name }: { folderId: FolderId; name: string }) =>
      setFolderName(folderId, name),
    onSuccess(folder) {
      invalidateFoldersList(queryClient, folder.profile_id);
      queryClient.setQueryData(
        foldersKeys.specific(folder.profile_id, folder.id),
        folder,
      );
    },
  }));
}

export function createSetFolderConfigMutation() {
  const queryClient = getQueryClientContext();

  return createMutation(() => ({
    mutationFn: ({
      folderId,
      config,
    }: {
      folderId: FolderId;
      config: FolderConfig;
    }) => setFolderConfig(folderId, config),
    onSuccess(folder) {
      invalidateFoldersList(queryClient, folder.profile_id);
      queryClient.setQueryData(
        foldersKeys.specific(folder.profile_id, folder.id),
        folder,
      );
    },
  }));
}

export function createDeleteFolderMutation() {
  const queryClient = getQueryClientContext();

  return createMutation(() => ({
    mutationFn: ({ folderId }: { folderId: FolderId; profileId: ProfileId }) =>
      deleteFolder(folderId),
    onSuccess(_, { profileId }) {
      invalidateFoldersList(queryClient, profileId);
    },
  }));
}
