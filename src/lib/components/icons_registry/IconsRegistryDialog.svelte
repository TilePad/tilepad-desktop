<script lang="ts">
  import type { IconRegistryEntry } from "$lib/api/types/icons_registry";

  import { createIconPacksQuery } from "$lib/api/icons";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { createIconPackRegistryQuery } from "$lib/api/icons_registry";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Dialog from "../dialog/Dialog.svelte";
  import IconsRegistryItem from "./IconsRegistryItem.svelte";
  import IconsRegistryViewer from "./IconsRegistryViewer.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & {};

  const { ...restProps }: Props = $props();

  const iconRegistryQuery = createIconPackRegistryQuery();
  const iconPacksQuery = createIconPacksQuery();

  let active: IconRegistryEntry | undefined = $state(undefined);
</script>

<Dialog {...restProps}>
  {#snippet children()}
    <div class="content">
      {#if $iconRegistryQuery.isLoading || $iconPacksQuery.isLoading}
        Loading...
      {:else if $iconRegistryQuery.isError}
        Failed to load community plugins: {getErrorMessage(
          $iconRegistryQuery.error,
        )}
      {:else if $iconPacksQuery.isError}
        Failed to load installed plugins: {getErrorMessage(
          $iconPacksQuery.error,
        )}
      {:else if $iconRegistryQuery.isSuccess && $iconPacksQuery.isSuccess}
        <div class="split">
          <div class="plugins">
            <div class="titlebar">
              <div class="titlebar__text">
                <h2>Community Icons</h2>
                <p class="total">{$iconRegistryQuery.data.length} Icon Packs</p>
              </div>

              <DialogCloseButton buttonLabel={{ text: "Close" }} />
            </div>

            <div class="plugins-list">
              {#each $iconRegistryQuery.data as item}
                <IconsRegistryItem
                  {item}
                  onClick={() => {
                    if (active !== undefined && active.id === item.id) {
                      active = undefined;
                    } else {
                      active = item;
                    }
                  }}
                  installed={$iconPacksQuery.data.find(
                    (plugin) => plugin.manifest.icons.id === item.id,
                  ) !== undefined}
                  selected={active !== undefined && active.id === item.id}
                />
              {:else}
                No plugins available
              {/each}
            </div>
          </div>

          <div class="viewer">
            {#if active}
              <IconsRegistryViewer
                item={active}
                installed={$iconPacksQuery.data.find(
                  (plugin) => plugin.manifest.icons.id === active!.id,
                ) !== undefined}
              />
            {/if}
          </div>
        </div>
      {/if}
    </div>
  {/snippet}
</Dialog>

<style>
  .content {
    max-width: 100%;
    width: 90vw;
    height: 90vh;

    display: flex;
    flex-flow: column;
    padding-bottom: 0;
  }

  .split {
    display: flex;
    flex-flow: row;
    height: 100%;
    overflow: hidden;
  }

  .plugins {
    display: flex;
    flex-flow: column;

    height: 100%;
    overflow: auto;
    max-width: 20rem;
    flex: auto;
    flex-shrink: 0;
    padding: 1rem;
    gap: 0.5rem;
  }

  .viewer {
    height: 100%;
    overflow: auto;
    flex: auto;
  }

  .plugins-list {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.5rem;
    flex-direction: column;
    flex: auto;
    overflow: auto;
  }

  .titlebar {
    display: flex;
    gap: 0.5rem;
    justify-content: space-between;
  }
  .total {
    font-size: 0.8rem;
  }
</style>
