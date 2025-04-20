<script lang="ts">
  import Aside from "$lib/components/Aside.svelte";
  import { createPluginsQuery } from "$lib/api/plugins";
  import { getErrorMessage } from "$lib/api/utils/error";
  import SolarShopBoldDuotone from "~icons/solar/shop-bold-duotone";
  import PluginCard from "$lib/components/plugins/PluginCard.svelte";
  import ManualImportPlugin from "$lib/components/plugins/ManualImportPlugin.svelte";
  import PluginsRegistryDialog from "$lib/components/plugins_registry/PluginsRegistryDialog.svelte";

  const pluginsQuery = createPluginsQuery();
</script>

<div class="layout">
  {#if $pluginsQuery.isLoading}
    <div class="skeleton-list">
      <div class="skeleton" style="width: 80%; height: 1rem"></div>
      <div class="skeleton" style="width: 70%; height: 1rem"></div>
      <div class="skeleton" style="width: 30%; height: 1rem"></div>
    </div>
  {:else if $pluginsQuery.isError}
    <Aside severity="error" style="width: 100%">
      Failed to load plugins: {getErrorMessage($pluginsQuery.error)}
    </Aside>
  {:else if $pluginsQuery.isSuccess && $pluginsQuery.data.length > 0}
    <div class="header">
      <h2>Plugins</h2>
      <div class="actions">
        <PluginsRegistryDialog
          buttonLabel={{
            text: "Community Plugins",
            icon: SolarShopBoldDuotone,
          }}
        />
        <ManualImportPlugin />
      </div>
    </div>

    <div class="plugins-wrapper">
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
    height: 100%;
    overflow: hidden;

    display: flex;
    flex-flow: column;
  }

  .header {
    display: flex;
    flex-flow: row;
    flex-shrink: 0;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem;
    border-bottom: 1px solid #333;
    background-color: #29262e;
  }

  .plugins-wrapper {
    flex: auto;
    overflow: auto;
  }

  .plugins {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    padding: 1rem;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }
</style>
