<script lang="ts">
  import type { ProfileModel } from "$lib/api/types/profiles";

  import { Select } from "bits-ui";
  import { slide } from "svelte/transition";
  import { createFoldersQuery } from "$lib/api/folders";
  import SolarAltArrowUpBold from "~icons/solar/alt-arrow-up-bold";
  import SolarAltArrowDownBold from "~icons/solar/alt-arrow-down-bold";
  import SolarFolder2BoldDuotone from "~icons/solar/folder-2-bold-duotone";

  import { getFolderContext } from "./FolderProvider.svelte";
  import CreateFolderDialog from "./CreateFolderDialog.svelte";
  import FolderSelectorSettings from "./FolderSelectorSettings.svelte";
  import { getProfileContext } from "../profiles/ProfilesProvider.svelte";

  const { profile } = getProfileContext();
  const { folder, setFolderId } = getFolderContext();

  const currentProfile = $derived.by(profile);
  const currentFolder = $derived.by(folder);

  const foldersQuery = createFoldersQuery(() => currentProfile.id);
  const folders = $derived($foldersQuery.data ?? []);

  let open = $state(false);
</script>

{#snippet item(profile: ProfileModel)}
  <span>{profile.name} </span>
{/snippet}

<Select.Root
  allowDeselect={false}
  type="single"
  onOpenChange={(value) => (open = value)}
  value={currentFolder?.id}
  onValueChange={(value) => setFolderId(value)}
>
  <Select.Trigger>
    {#snippet child({ props })}
      <div class="trigger-wrapper">
        <button class="trigger" {...props}>
          <SolarFolder2BoldDuotone />
          {currentFolder.name}

          {#if open}
            <SolarAltArrowUpBold />
          {:else}
            <SolarAltArrowDownBold />
          {/if}
        </button>

        <FolderSelectorSettings folder={currentFolder} />
        <CreateFolderDialog order={folders.length} />
      </div>
    {/snippet}
  </Select.Trigger>

  <Select.Portal>
    <Select.Content sideOffset={8} align="start">
      {#snippet child({ props, open, wrapperProps })}
        <div {...wrapperProps} class="content-wrapper">
          {#if open}
            <div
              {...props}
              class="content"
              transition:slide={{ duration: 100 }}
            >
              {#each folders as value}
                <Select.Item value={value.id} label={value.name}>
                  {#snippet child({ props, selected, highlighted })}
                    <div
                      {...props}
                      class="item"
                      class:item--selected={selected}
                      class:item--highlighted={highlighted}
                    >
                      {@render item(value)}
                    </div>
                  {/snippet}
                </Select.Item>
              {/each}
            </div>
          {/if}
        </div>
      {/snippet}
    </Select.Content>
  </Select.Portal>
</Select.Root>

<style>
  .content {
    border-radius: 0.5rem;
    background-color: #2a2631;
    padding: 0.25rem;
    max-height: 40vh;
    min-width: 20rem;
    overflow: auto;
    box-shadow: 2px 10px 20px rgba(0, 0, 0, 0.5);
  }

  .form-input {
    display: inline-flex;
    flex-flow: column;
  }

  .item {
    cursor: pointer;
    display: flex;
    border-radius: 0.25rem;
    padding: 0.5rem;
    max-width: 500px;
  }

  .item:hover {
    background-color: #706580;
  }

  .item--selected {
    background-color: #675d75;
  }

  .item--highlighted {
    outline: 1px solid white;
  }

  .trigger {
    padding: 0.5rem;
    border: none;
    background-color: #1f1d22;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;
    justify-content: space-between;
  }

  .trigger-wrapper {
    display: flex;
  }
</style>
