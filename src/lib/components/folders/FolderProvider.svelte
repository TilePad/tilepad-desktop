<script module>
  const FOLDER_STORE_KEY = Symbol("FolderStore");
  const currentFolderKey = "currentFolderId";

  export function getPersistedFolderId() {
    return localStorage.getItem(currentFolderKey) ?? undefined;
  }

  function setPersistedFolderId(profileId: string) {
    localStorage.setItem(currentFolderKey, profileId);
  }

  interface FolderContext {
    folder(): FolderModel;
    folderId(): FolderId;
    setFolderId: (value: FolderId) => void;
  }

  export function getFolderContext(): FolderContext {
    return getContext(FOLDER_STORE_KEY);
  }
</script>

<!-- Provider for profiles, loads the active default profile, creates one if missing -->
<script lang="ts">
  import type { FolderId, FolderModel } from "$lib/api/types/folders";

  import { watch } from "runed";
  import { t } from "svelte-i18n";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { getContext, setContext, type Snippet } from "svelte";
  import {
    createFolderQuery,
    createFoldersQuery,
    createCreateFolderMutation,
  } from "$lib/api/folders";

  import Aside from "../Aside.svelte";
  import SkeletonList from "../skeleton/SkeletonList.svelte";
  import {
    getProfileContext,
    getPersistedProfileId,
  } from "../profiles/ProfilesProvider.svelte";

  type Props = {
    content?: Snippet<[{ folder: FolderModel }]> | undefined;
  };

  const { content }: Props = $props();

  const profileContext = getProfileContext();
  const currentProfile = $derived.by(profileContext.profile);

  const foldersQuery = createFoldersQuery(() => currentProfile.id);
  const foldersQueryData = $derived(foldersQuery.data);

  // State for the actively selected folder
  let folderId: FolderId | undefined = $state(getPersistedCurrentFolder());

  const folderQuery = createFolderQuery(
    () => currentProfile.id,
    () => folderId ?? null,
  );

  const folder = $derived(folderQuery.data);

  const createFolder = createCreateFolderMutation();

  function getPersistedCurrentFolder() {
    const profile = profileContext.profile();
    const currentProfileId = getPersistedProfileId();
    if (currentProfileId === undefined || currentProfileId !== profile.id)
      return undefined;
    return getPersistedFolderId();
  }

  function getDefaultFolder(folders: FolderModel[]): FolderModel | undefined {
    return folders.find((profile) => profile.default);
  }

  setContext(FOLDER_STORE_KEY, {
    folder: () => folder!,
    folderId: () => folderId!,
    setFolderId: (value: FolderId) => {
      folderId = value;
      setPersistedFolderId(value);
    },
  });

  watch(
    () => foldersQueryData,
    (folders) => {
      // Profiles are loaded yet ignore
      if (folders === undefined) return;

      // Check if the current folder is a valid profile to use
      if (folders.length > 0 && folderId !== undefined) {
        const currentFolder = folders.find((folder) => folder.id === folderId);
        if (currentFolder !== undefined) {
          return;
        }
      }

      // Try and set the profile to the default
      if (folders.length > 0) {
        const defaultFolder = getDefaultFolder(folders);
        if (defaultFolder !== undefined) folderId = defaultFolder.id;
      }

      // Default profile is set
      if (folderId !== undefined) return;

      // Create a new default profile
      createFolder.mutate(
        {
          create: {
            name: $t("default_folder"),
            default: true,
            config: {},
            profile_id: currentProfile.id,
            order: 0,
          },
        },
        {
          // Use the newly create profile
          onSuccess: (data) => {
            folderId = data.id;
          },
        },
      );
    },
  );
</script>

{#if createFolder.isPending || foldersQuery.isLoading || folderQuery.isLoading}
  <!-- Loading states -->
  <SkeletonList style="margin: 1rem;" />
{:else if createFolder.isError}
  <!-- Error creating current folder -->
  <Aside severity="error" style="margin: 1rem;">
    {$t("create_folder_error", {
      values: { error: getErrorMessage(createFolder.error) },
    })}
  </Aside>
{:else if foldersQuery.isError}
  <!-- Error loading folders list -->
  <Aside severity="error" style="margin: 1rem;">
    {$t("folders_error", {
      values: { error: getErrorMessage(foldersQuery.error) },
    })}
  </Aside>
{:else if folderQuery.isError}
  <!-- Error loading current folder -->
  <Aside severity="error" style="margin: 1rem;">
    {$t("folder_error", {
      values: { error: getErrorMessage(folderQuery.error) },
    })}
  </Aside>
{:else if (createFolder.isIdle || createFolder.isSuccess) && foldersQuery.isSuccess && folderQuery.isSuccess && folder}
  {@render content?.({ folder })}
{/if}
