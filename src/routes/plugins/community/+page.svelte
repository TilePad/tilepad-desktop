<script lang="ts">
  import type { PluginWithState } from "$lib/api/types/plugin";

  import { toast } from "svelte-sonner";
  import Aside from "$lib/components/Aside.svelte";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { compare as semverCompare } from "semver-ts";
  import { createPluginsQuery } from "$lib/api/plugins";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { createMutation } from "@tanstack/svelte-query";
  import Button from "$lib/components/input/Button.svelte";
  import SolarShopBoldDuotone from "~icons/solar/shop-bold-duotone";
  import SolarBoxBoldDuotone from "~icons/solar/box-bold-duotone";
  import {
    createPluginRegistryQuery,
    getLatestPluginVersions,
  } from "$lib/api/plugins_registry";
  import SkeletonList from "$lib/components/skeleton/SkeletonList.svelte";
  import { getSettingsContext } from "$lib/components/SettingsProvider.svelte";
  import ManualImportPlugin from "$lib/components/plugins/ManualImportPlugin.svelte";
  import { serverContext } from "$lib/contexts/server.context";
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";
  import PluginsRegistryItem from "$lib/components/plugins_registry/PluginsRegistryItem.svelte";
  import PluginRegistryViewer from "$lib/components/plugins_registry/PluginRegistryViewer.svelte";

  const i18n = i18nContext.get();

  const settingsContext = getSettingsContext();
  const settings = $derived.by(settingsContext.settings);

  const currentServerContext = serverContext.get();

  const pluginRegistryQuery = createPluginRegistryQuery();
  const pluginsQuery = createPluginsQuery();

  let active: PluginRegistryEntry | undefined = $state(undefined);
  let search = $state("");

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

  const checkUpdatesMutation = createMutation(() => ({
    mutationFn: async ({ plugins }: { plugins: PluginWithState[] }) => {
      const latestVersions = await getLatestPluginVersions(plugins);
      const updates = [];

      for (const entry of latestVersions) {
        const localPlugin = plugins.find(
          (plugin) => plugin.manifest.plugin.id === entry.manifest.plugin.id,
        );

        if (
          localPlugin &&
          semverCompare(
            entry.manifest.plugin.version,
            localPlugin.manifest.plugin.version,
          ) === 1
        ) {
          updates.push(entry);
        }
      }

      toast.success(
        i18n.f("updates_found_count", { values: { count: updates.length } }),
      );

      return updates;
    },
  }));
</script>

<div class="layout">
  {#if pluginRegistryQuery.isLoading || pluginsQuery.isLoading}
    <SkeletonList style="margin: 1rem" />
  {:else if pluginsQuery.isError}
    <Aside severity="error" style="margin: 1rem">
      {i18n.f("plugins_error", {
        values: { error: getErrorMessage(pluginsQuery.error) },
      })}
    </Aside>
  {:else if pluginRegistryQuery.isSuccess && pluginsQuery.isSuccess}
    <div class="header">
      <div class="nav">
        <a class="tab" href="/plugins">
          <SolarBoxBoldDuotone />

          {i18n.f("installed")}
        </a>
        <a class="tab tab--active" href="/plugins/community">
          <SolarShopBoldDuotone />

          {i18n.f("community_plugins")}
        </a>
      </div>

      <input
        bind:value={search}
        class="search"
        type="text"
        placeholder={i18n.f("search_placeholder")}
      />

      <div class="actions">
        <Button
          variant="secondary"
          onclick={() => {
            checkUpdatesMutation.mutate({ plugins: pluginsQuery.data });
          }}
          loading={checkUpdatesMutation.isPending}
        >
          {i18n.f("check_for_updates")}
        </Button>

        <ManualImportPlugin />
      </div>
    </div>

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
                active = undefined;
              } else {
                active = item;
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
          <PluginRegistryViewer
            item={active}
            installed={pluginsQuery.data.find(
              (plugin) => plugin.manifest.plugin.id === active!.id,
            )?.manifest}
          />
        {/if}
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
    padding: var(--tp-space-4);
    display: flex;
    gap: var(--tp-space-4);
    justify-content: space-between;
  }

  .viewer {
    height: 100%;
    overflow: auto;
    flex: auto;
  }

  .search {
    padding: 0.5rem;
    background-color: #1f1d22;
    border: 1px solid #666;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    flex: auto;
  }

  .plugins-list {
    display: flex;
    gap: 0.5rem;
    flex-direction: column;
    flex: auto;
    overflow: auto;
  }

  .plugins-wrapper {
    display: flex;
    flex-flow: row;
    height: 100%;
    overflow: hidden;
  }

  .plugins-list {
    display: flex;
    gap: 0.5rem;
    flex-direction: column;
    flex: auto;
    overflow: auto;
    max-width: 24rem;
    padding: 0 1rem;
  }

  .actions {
    display: flex;
    gap: var(--tp-space-3);
  }

  .nav {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .tab {
    display: inline-flex;
    align-items: center;
    gap: var(--tp-space-2);
    justify-content: center;
    font-size: var(--tp-text-base);
    font-weight: var(--tp-font-weight-medium);
    text-decoration: none;
    color: var(--tp-text-primary);
    border-bottom: 2px solid transparent;
    height: var(--tp-btn-height-md);
    padding: 0 var(--tp-btn-padding-x-md);
    border-radius: var(--tp-radius-md);
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
  }

  .tab--active {
    background: var(--tp-bg-tertiary);
    border-bottom: 2px solid var(--tp-text-primary);
  }
</style>
