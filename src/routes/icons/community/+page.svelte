<script lang="ts">
  import type { IconRegistryEntry } from "$lib/api/types/icons_registry";

  import Aside from "$lib/components/Aside.svelte";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { createIconPacksQuery } from "$lib/api/icons";
  import { getErrorMessage } from "$lib/api/utils/error";
  import SolarBoxBoldDuotone from "~icons/solar/box-bold-duotone";
  import SolarShopBoldDuotone from "~icons/solar/shop-bold-duotone";
  import { createIconPackRegistryQuery } from "$lib/api/icons_registry";
  import SkeletonList from "$lib/components/skeleton/SkeletonList.svelte";
  import IconsRegistryItem from "$lib/components/icons_registry/IconsRegistryItem.svelte";
  import IconsRegistryViewer from "$lib/components/icons_registry/IconsRegistryViewer.svelte";

  const i18n = i18nContext.get();

  const iconRegistryQuery = createIconPackRegistryQuery();
  const iconPacksQuery = createIconPacksQuery();

  let active: IconRegistryEntry | undefined = $state(undefined);
  let search = $state("");

  const filteredRegistry = $derived(
    filterIconPacks(iconRegistryQuery.data ?? [], search),
  );

  function filterIconPacks(packs: IconRegistryEntry[], query: string) {
    query = query.toLowerCase();

    if (query.length < 1) return packs;

    return packs.filter((entry) => {
      const name = entry.name.toLowerCase();
      return name === query || name.includes(query);
    });
  }
</script>

<div class="layout">
  <div class="header">
    <div class="nav">
      <a class="tab" href="/icons">
        <SolarBoxBoldDuotone />

        {i18n.f("installed")}
      </a>
      <a class="tab tab--active" href="/icons/community">
        <SolarShopBoldDuotone />
        {i18n.f("community_icon_packs")}
      </a>
    </div>

    <input
      bind:value={search}
      class="search"
      type="text"
      placeholder={i18n.f("search_placeholder")}
    />
  </div>

  {#if iconPacksQuery.isLoading}
    <SkeletonList style="margin: 1rem" />
  {:else if iconPacksQuery.isError}
    <Aside severity="error" style="margin: 1rem">
      {i18n.f("icon_packs_installed_error", {
        values: { error: getErrorMessage(iconPacksQuery.error) },
      })}
    </Aside>
  {:else if iconPacksQuery.isSuccess}
    <div class="plugins-wrapper">
      <div class="plugins-list">
        {#each filteredRegistry as item (item.id)}
          <IconsRegistryItem
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
            installed={iconPacksQuery.data.find(
              (plugin) => plugin.manifest.icons.id === item.id,
            ) !== undefined}
            selected={active !== undefined && active.id === item.id}
          />
        {:else}
          {i18n.f("community_icons_none")}
        {/each}
      </div>

      <div class="viewer">
        {#if active}
          <IconsRegistryViewer
            item={active}
            installed={iconPacksQuery.data.find(
              (plugin) => plugin.manifest.icons.id === active!.id,
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

  .nav {
    display: flex;
    align-items: center;
    flex-shrink: 0;
    width: calc(24rem - var(--tp-space-4) * 2);
  }

  .tab {
    display: inline-flex;
    align-items: center;
    flex: auto;
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
