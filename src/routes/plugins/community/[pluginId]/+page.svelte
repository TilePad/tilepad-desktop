<script lang="ts">
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import Aside from "$lib/components/Aside.svelte";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { createPluginsQuery } from "$lib/api/plugins";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { createPluginRegistryQuery } from "$lib/api/plugins_registry";
  import SkeletonList from "$lib/components/skeleton/SkeletonList.svelte";
  import PluginsRegistryItem from "$lib/components/plugins_registry/PluginsRegistryItem.svelte";
  import PluginRegistryViewer from "$lib/components/plugins_registry/PluginRegistryViewer.svelte";

  import { pluginSearchContext } from "../+layout.svelte";

  const activeId = $derived(page.params.pluginId);

  const i18n = i18nContext.get();
  const pluginSearch = pluginSearchContext.get();

  const pluginRegistryQuery = createPluginRegistryQuery();
  const pluginsQuery = createPluginsQuery();

  const search = $derived(pluginSearch.query);
  const active = $derived.by(() => {
    if (!activeId || !pluginRegistryQuery.data) return undefined;

    return pluginRegistryQuery.data.find((plugin) => plugin.id === activeId);
  });

  const filteredRegistry = $derived(
    filterIconPacks(pluginRegistryQuery.data ?? [], search),
  );

  function filterIconPacks(packs: PluginRegistryEntry[], query: string) {
    query = query.toLowerCase();

    if (query.length < 1) return packs;

    return packs.filter((entry) => {
      const name = entry.name.toLowerCase();
      return name === query || name.includes(query);
    });
  }
</script>

{#if pluginRegistryQuery.isLoading || pluginsQuery.isLoading}
  <SkeletonList style="margin: 1rem" />
{:else if pluginsQuery.isError}
  <Aside severity="error" style="margin: 1rem">
    {i18n.f("plugins_error", {
      values: { error: getErrorMessage(pluginsQuery.error) },
    })}
  </Aside>
{:else if pluginRegistryQuery.isError}
  <Aside severity="error" style="margin: 1rem">
    {i18n.f("community_plugins_error", {
      values: { error: getErrorMessage(pluginRegistryQuery.error) },
    })}
  </Aside>
{:else if pluginRegistryQuery.isSuccess && pluginsQuery.isSuccess}
  <div class="plugins-wrapper">
    <div class="plugins-list">
      {#each filteredRegistry as item (item.id)}
        <PluginsRegistryItem
          name={item.name}
          authors={item.authors}
          description={item.description}
          repo={item.repo}
          onClick={() => {
            if (active !== undefined && active.id === item.id) {
              goto(`/plugins/community`);
            } else {
              goto(`/plugins/community/${item.id}`);
            }
          }}
          installed={pluginsQuery.data.find(
            (plugin) => plugin.manifest.plugin.id === item.id,
          ) !== undefined}
          selected={active !== undefined && active.id === item.id}
        />
      {:else}
        {i18n.f("community_plugins_none")}
      {/each}
    </div>

    <div class="viewer">
      {#if active}
        {#key active.id}
          <PluginRegistryViewer
            item={active}
            installed={pluginsQuery.data.find(
              (plugin) => plugin.manifest.plugin.id === active!.id,
            )?.manifest}
          />
        {/key}
      {/if}
    </div>
  </div>
{/if}

<style>
  .viewer {
    height: 100%;
    overflow: auto;
    flex: auto;
  }

  .plugins-list {
    display: flex;
    gap: 0.5rem;
    flex-direction: column;
    flex: auto;
    overflow: auto;
    height: 100%;
  }

  .plugins-wrapper {
    width: 100%;
    display: grid;
    grid-template-columns: 24rem 1fr;
    height: 100%;
    overflow: hidden;
  }

  .plugins-list {
    display: block;
    gap: 0.5rem;
    overflow: auto;
    width: 100%;
    padding: 0 1rem;
    height: 100%;
  }

  .plugins-list:global(> .item) {
    margin-bottom: 1rem;
  }
</style>
