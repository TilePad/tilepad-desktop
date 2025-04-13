<script lang="ts">
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  import { getErrorMessage } from "$lib/api/utils/error";
  import { createPluginRegistryQuery } from "$lib/api/plugins_registry";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Dialog from "../dialog/Dialog.svelte";
  import PluginsRegistryItem from "./PluginsRegistryItem.svelte";
  import PluginRegistryViewer from "./PluginRegistryViewer.svelte";

  type Props = DialogProps & {};

  const { ...restProps }: Props = $props();

  const pluginRegistryQuery = createPluginRegistryQuery();

  let active: PluginRegistryEntry | undefined = $state(undefined);
</script>

<Dialog {...restProps}>
  {#snippet children()}
    <div class="content">
      <h1>Community Plugins</h1>

      {#if $pluginRegistryQuery.isLoading}
        Loading...
      {:else if $pluginRegistryQuery.isError}
        Failed to load community plugins: {getErrorMessage(
          $pluginRegistryQuery.error,
        )}
      {:else if $pluginRegistryQuery.isSuccess}
        <div class="split">
          <div class="plugins">
            {#each $pluginRegistryQuery.data as item}
              <PluginsRegistryItem {item} onClick={() => (active = item)} />
            {:else}
              No plugins available
            {/each}
          </div>

          <div class="viewer">
            {#if active}
              <PluginRegistryViewer item={active} />
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
    padding: 1rem;
    padding-bottom: 0;
  }

  .split {
    display: flex;
    flex-flow: row;
  }
</style>
