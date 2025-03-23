<script lang="ts">
  import { createPluginsQuery } from "$lib/api/plugins";
  import PluginCard from "$lib/components/plugins/PluginCard.svelte";

  const pluginsQuery = createPluginsQuery();
</script>

<div class="layout">
  {#if $pluginsQuery.isLoading}
    Loading plugins...
  {:else if $pluginsQuery.isError}
    Failed to load plugins {$pluginsQuery.error}
  {:else if $pluginsQuery.isSuccess && $pluginsQuery.data.length > 0}
    <div class="section">
      <h2>Plugins</h2>
      <div class="plugins">
        {#each $pluginsQuery.data as plugin}
          {#if !plugin.manifest.plugin.internal}
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
</style>
