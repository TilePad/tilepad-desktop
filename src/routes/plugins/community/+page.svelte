<script lang="ts">
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  import { goto } from "$app/navigation";
  import Aside from "$lib/components/Aside.svelte";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { createPluginsQuery } from "$lib/api/plugins";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { createPluginRegistryQuery } from "$lib/api/plugins_registry";
  import SkeletonList from "$lib/components/skeleton/SkeletonList.svelte";
  import PluginsRegistryItem from "$lib/components/plugins_registry/PluginsRegistryItem.svelte";

  import { pluginSearchContext } from "./+layout.svelte";

  const i18n = i18nContext.get();
  const pluginSearch = pluginSearchContext.get();

  const pluginRegistryQuery = createPluginRegistryQuery();
  const pluginsQuery = createPluginsQuery();

  const search = $derived(pluginSearch.query);

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
  <div class="plugins-list">
    {#each filteredRegistry as item (item.id)}
      <PluginsRegistryItem
        name={item.name}
        authors={item.authors}
        description={item.description}
        repo={item.repo}
        onClick={() => {
          goto(`/plugins/community/${item.id}`);
        }}
        installed={pluginsQuery.data.find(
          (plugin) => plugin.manifest.plugin.id === item.id,
        ) !== undefined}
        selected={false}
      />
    {:else}
      {i18n.f("community_plugins_none")}
    {/each}
  </div>
{/if}

<style>
  .plugins-list {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(
      auto-fit,
      minmax(min(400px, max(200px, 100%)), 1fr)
    );
    gap: var(--tp-space-4);
    padding: var(--tp-space-4);
    padding-top: 0;
  }
</style>
