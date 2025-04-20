<script lang="ts">
  import { createPluginsQuery } from "$lib/api/plugins";
  import PluginCard from "$lib/components/plugins/PluginCard.svelte";
  import ManualImportPlugin from "$lib/components/plugins/ManualImportPlugin.svelte";
  import PluginsRegistryDialog from "$lib/components/plugins_registry/PluginsRegistryDialog.svelte";

  const pluginsQuery = createPluginsQuery();
</script>

<div class="layout">
  {#if $pluginsQuery.isLoading}
    Loading plugins...
  {:else if $pluginsQuery.isError}
    Failed to load plugins {$pluginsQuery.error}
  {:else if $pluginsQuery.isSuccess && $pluginsQuery.data.length > 0}
    <div class="section">
      <div class="header">
        <h2>Plugins</h2>
        <div class="actions">
          <PluginsRegistryDialog buttonLabel={{ text: "Community Plugins" }} />
          <ManualImportPlugin />
        </div>
      </div>

      <div class="plugins">
        {#each $pluginsQuery.data as plugin (plugin.manifest.plugin.id)}
          {#if !plugin.manifest.plugin.internal || import.meta.env.DEV}
            <PluginCard {plugin} />
          {/if}
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .layout {
    flex: auto;

    display: flex;
    flex-flow: column;
    gap: 0.5rem;

    padding: 1rem;
  }

  .header {
    display: flex;
    flex-flow: row;
    justify-content: space-between;
    align-items: center;
  }

  .section {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
  }

  .plugins {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }
</style>
