<script module>
  const FOLDER_STORE_KEY = Symbol("FolderStore");

  interface FolderContext {
    folder(): FolderModel;
    setFolder: (value: FolderModel) => void;
  }

  export function getFolderContext(): FolderContext {
    return getContext(FOLDER_STORE_KEY);
  }
</script>

<!-- Provider for profiles, loads the active default profile, creates one if missing -->
<script lang="ts">
  import type { FolderModel } from "$lib/api/types/folders";

  import { watch } from "runed";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { getContext, setContext, type Snippet } from "svelte";
  import {
    createFoldersQuery,
    createCreateFolderMutation,
  } from "$lib/api/folders";

  import { getProfileContext } from "./ProfilesProvider.svelte";

  type Props = {
    children?: Snippet | undefined;
  };

  const { children }: Props = $props();

  const profileContext = getProfileContext();
  const currentProfile = $derived(profileContext.profile());

  const foldersQuery = createFoldersQuery(() => currentProfile.id);
  const foldersQueryData = $derived($foldersQuery.data);

  const createFolder = createCreateFolderMutation();

  // State for the actively selected folder
  let folder: FolderModel | undefined = $state(undefined);

  function getDefaultFolder(folders: FolderModel[]): FolderModel | undefined {
    return folders.find((profile) => profile.default);
  }

  setContext(FOLDER_STORE_KEY, {
    folder: () => folder,
    setFolder: (value: FolderModel) => (folder = value),
  });

  watch(
    () => foldersQueryData,
    (folders) => {
      // Profiles are loaded yet ignore
      if (folders === undefined) return;

      // Try and set the profile to the default
      if (folders.length > 0) {
        folder = getDefaultFolder(folders);
      }

      // Default profile is set
      if (folder !== undefined) return;

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
            folder = data;
          },
        },
      );
    },
  );
</script>

{#if $createFolder.isIdle || $createFolder.isSuccess}
  {#if $foldersQuery.isLoading}
    Loading...
  {:else if $foldersQuery.isError}
    Failed to load profiles {getErrorMessage($foldersQuery.error)}
  {:else if $foldersQuery.isSuccess}
    {#if folder}
      {@render children?.()}
    {/if}
  {/if}
{:else if $createFolder.isPending}
  Creating default profile...
{:else if $createFolder.isError}
  Failed to create default profile {getErrorMessage($createFolder.error)}
{/if}
