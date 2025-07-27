<script lang="ts">
  import { t } from "svelte-i18n";
  import { Select } from "bits-ui";
  import { slide } from "svelte/transition";
  import { createFoldersQuery } from "$lib/api/folders";
  import DownArrow from "~icons/solar/alt-arrow-down-bold";
  import { type FolderModel } from "$lib/api/types/folders";
  import SolarFolder2BoldDuotone from "~icons/solar/folder-2-bold-duotone";

  import Button from "../input/Button.svelte";
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

{#snippet item(folder: FolderModel)}
  <span>{folder.name}</span>

  {#if folder.default}
    <span class="default-label">
      {$t("default")}
    </span>
  {/if}
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
      <div class="wrapper" data-open={open}>
        <Button variant="secondary" class="trigger" {...props}>
          <SolarFolder2BoldDuotone />
          {currentFolder.name}

          <DownArrow class="trigger__icon" />
        </Button>

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
              {#each folders as value (value.id)}
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
    gap: 0.5rem;
    border-radius: 0.25rem;
    padding: 0.5rem;
    max-width: 500px;
    align-items: center;
    justify-content: space-between;
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

  .default-label {
    padding: 0.25rem 0.5rem;
    background-color: #141316;
    border-radius: 0.25rem;
  }

  .wrapper {
    display: flex;
  }

  .wrapper:global(> .trigger) {
    justify-content: space-between;
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }

  .wrapper:global(> .trigger > .trigger__icon) {
    transition: all var(--tp-transition-fast);
    transform-origin: center;
  }

  .wrapper[data-open="true"]:global(> .trigger > .trigger__icon) {
    transform: rotate(-180deg);
  }
</style>
