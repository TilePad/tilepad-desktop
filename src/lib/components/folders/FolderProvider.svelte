<script module>
  const FOLDER_STORE_KEY = Symbol("FolderStore");

  interface FolderContext {
    folder(): FolderModel;
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
  import { getErrorMessage } from "$lib/api/utils/error";
  import { getContext, setContext, type Snippet } from "svelte";
  import {
    createFolderQuery,
    createFoldersQuery,
    createCreateFolderMutation,
  } from "$lib/api/folders";

  import { getProfileContext } from "../profiles/ProfilesProvider.svelte";

  type Props = {
    children?: Snippet | undefined;
  };

  const { children }: Props = $props();

  const profileContext = getProfileContext();
  const currentProfile = $derived.by(profileContext.profile);

  const foldersQuery = createFoldersQuery(() => currentProfile.id);
  const foldersQueryData = $derived($foldersQuery.data);

  // State for the actively selected folder
  let folderId: FolderId | undefined = $state(undefined);

  const folderQuery = createFolderQuery(
    () => currentProfile.id,
    () => folderId ?? null,
  );

  const folder = $derived($folderQuery.data);

  const createFolder = createCreateFolderMutation();

  function getDefaultFolder(folders: FolderModel[]): FolderModel | undefined {
    return folders.find((profile) => profile.default);
  }

  setContext(FOLDER_STORE_KEY, {
    folder: () => folder!,
    setFolderId: (value: FolderId) => (folderId = value),
  });

  watch(
    () => foldersQueryData,
    (folders) => {
      // Profiles are loaded yet ignore
      if (folders === undefined) return;

      // Try and set the profile to the default
      if (folders.length > 0) {
        const defaultFolder = getDefaultFolder(folders);
        if (defaultFolder !== undefined) folderId = defaultFolder.id;
      }

      // Default profile is set
      if (folderId !== undefined) return;

      // Create a new default profile
      $createFolder.mutate(
        {
          create: {
            name: "Default Folder",
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

<!-- Query create default -->
{#if $createFolder.isIdle || $createFolder.isSuccess}
  <!-- Query folders -->
  {#if $foldersQuery.isLoading}
    Loading folders...
  {:else if $foldersQuery.isError}
    Failed to load profiles {getErrorMessage($foldersQuery.error)}
  {:else if $foldersQuery.isSuccess}
    <!-- Query the current folder -->
    {#if $folderQuery.isLoading}
      Loading folder...
    {:else if $folderQuery.isError}
      Failed to load folder {getErrorMessage($folderQuery.error)}
    {:else if $folderQuery.isSuccess}
      {@render children?.()}
    {/if}
  {/if}
{:else if $createFolder.isPending}
  Creating default profile...
{:else if $createFolder.isError}
  Failed to create default profile {getErrorMessage($createFolder.error)}
{/if}
