<script>
  import { createFoldersQuery } from "$lib/api/folders";
  import { getErrorMessage } from "$lib/api/utils/error";
  import SolarMenuDotsBold from "~icons/solar/menu-dots-bold";
  import SolarFolder2BoldDuotone from "~icons/solar/folder-2-bold-duotone";
  import SolarFolderOpenBoldDuotone from "~icons/solar/folder-open-bold-duotone";

  import PopoverButton from "../popover/PopoverButton.svelte";
  import { getProfileContext } from "./ProfilesProvider.svelte";
  import EditFolderDialog from "../folders/EditFolderDialog.svelte";
  import { getFolderContext } from "../folders/FolderProvider.svelte";
  import CreateFolderDialog from "../folders/CreateFolderDialog.svelte";
  import DeleteFolderDialog from "../folders/DeleteFolderDialog.svelte";
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
        <span class="folder__icon">
          {#if isCurrent}
            <SolarFolderOpenBoldDuotone />
          {:else}
            <SolarFolder2BoldDuotone />
          {/if}</span
        >

        <span class="folder__name">
          {folder.name}
        </span>

        <PopoverButton
          transparent
          onclick={(event) => event.stopPropagation()}
          style="flex-shrink: 0;"
        >
          {#snippet children()}<SolarMenuDotsBold />{/snippet}

          {#snippet content()}
            <EditFolderDialog {folder} />
            <DeleteFolderDialog {folder} />
          {/snippet}
        </PopoverButton>
      </button>
    {/each}

    <CreateFolderDialog order={folders.length} />
  </div>
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

  .folder__icon {
    flex-shrink: 0;
  }

  .folder__name {
    flex: auto;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
</style>
