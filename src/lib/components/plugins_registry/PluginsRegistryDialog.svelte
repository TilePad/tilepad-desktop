<script lang="ts">
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  import { createPluginsQuery } from "$lib/api/plugins";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { createPluginRegistryQuery } from "$lib/api/plugins_registry";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Dialog from "../dialog/Dialog.svelte";
  import PluginsRegistryItem from "./PluginsRegistryItem.svelte";
  import PluginRegistryViewer from "./PluginRegistryViewer.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & {};

  const { ...restProps }: Props = $props();

  const pluginRegistryQuery = createPluginRegistryQuery();
  const pluginsQuery = createPluginsQuery();

  let active: PluginRegistryEntry | undefined = $state(undefined);
</script>

<Dialog {...restProps}>
  {#snippet children()}
    <div class="content">
      {#if $pluginRegistryQuery.isLoading || $pluginsQuery.isLoading}
        Loading...
      {:else if $pluginRegistryQuery.isError}
        Failed to load community plugins: {getErrorMessage(
          $pluginRegistryQuery.error,
        )}
      {:else if $pluginsQuery.isError}
        Failed to load installed plugins: {getErrorMessage($pluginsQuery.error)}
      {:else if $pluginRegistryQuery.isSuccess && $pluginsQuery.isSuccess}
        <div class="split">
          <div class="plugins">
            <div>
              <h1>Community Plugins</h1>

              <DialogCloseButton buttonLabel={{ text: "Close" }} />
            </div>

            {#each $pluginRegistryQuery.data as item}
              <PluginsRegistryItem
                {item}
                onClick={() => (active = item)}
                installed={$pluginsQuery.data.find(
                  (plugin) => plugin.manifest.plugin.id === item.id,
                ) !== undefined}
              />
            {:else}
              No plugins available
            {/each}
          </div>

          <div class="viewer">
            {#if active}
              <PluginRegistryViewer
                item={active}
                installed={$pluginsQuery.data.find(
                  (plugin) => plugin.manifest.plugin.id === active!.id,
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
    max-width: 30rem;
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
</style>
