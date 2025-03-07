<script>
  import { createFoldersQuery } from "$lib/api/folders";
  import { getErrorMessage } from "$lib/api/utils/error";
  import SolarFolder2BoldDuotone from "~icons/solar/folder-2-bold-duotone";
  import SolarFolderOpenBoldDuotone from "~icons/solar/folder-open-bold-duotone";

  import { getProfileContext } from "./ProfilesProvider.svelte";
  import { getFolderContext } from "../folders/FolderProvider.svelte";
  import CreateFolderDialog from "../folders/CreateFolderDialog.svelte";

  const { profile } = getProfileContext();
  const currentProfile = $derived.by(profile);

  const { folder, setFolderId } = getFolderContext();
  const currentFolder = $derived.by(folder);

  const foldersQuery = createFoldersQuery(() => currentProfile.id);
  const folders = $derived($foldersQuery.data ?? []);
</script>

{#if $foldersQuery.isLoading}
  Loading folders...
{:else if $foldersQuery.isError}
  Failed to load folders: {getErrorMessage($foldersQuery.error)}
{:else if $foldersQuery.isSuccess}
  <div class="folders">
    {#each folders as folder}
      {@const isCurrent = currentFolder.id === folder.id}
      <button
        class="folder"
        class:folder--current={isCurrent}
        onclick={() => {
          setFolderId(folder.id);
        }}
      >
        {#if isCurrent}
          <SolarFolderOpenBoldDuotone />
        {:else}
          <SolarFolder2BoldDuotone />
        {/if}
        {folder.name}
      </button>
    {/each}
  </div>
  <CreateFolderDialog
    order={folders.length}
    buttonLabel={{ text: "Create Folder" }}
  />
{/if}

<style>
  .folders {
    display: flex;
    flex-flow: column;
    margin-bottom: 1rem;
  }

  .folder {
    display: flex;
    gap: 0.5rem;
    background-color: #413c49;
    color: #fff;
    border: none;
    width: 100%;
    text-align: left;
    padding: 0.5rem;
    cursor: pointer;
    background-color: #1a181d;
    font-size: 0.9rem;
  }

  .folder--current {
    background-color: #604a85;
  }
</style>
