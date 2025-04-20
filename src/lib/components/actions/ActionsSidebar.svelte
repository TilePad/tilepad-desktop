<script lang="ts">
  import type { PluginId } from "$lib/api/types/plugin";
  import type { ActionCategory } from "$lib/api/types/actions";

  import { createActionsQuery } from "$lib/api/actions";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { persistedState } from "$lib/utils/localStorage.svelte";
  import SolarAltArrowLeftOutline from "~icons/solar/alt-arrow-left-outline";
  import SolarAltArrowRightOutline from "~icons/solar/alt-arrow-right-outline";

  import ActionsSidebarCategory from "./ActionCategory.svelte";

  type ActionSidebarState = {
    expanded?: boolean;
    expandedCategories?: Partial<Record<PluginId, boolean>>;
  };

  const actionsQuery = createActionsQuery();

  const actionSidebarState = persistedState<ActionSidebarState>(
    "actionSidebarState",
    { expanded: true, expandedCategories: {} },
  );

  const sidebarExpanded = $derived(actionSidebarState.current.expanded ?? true);

  let search = $state("");

  const filteredCategories = $derived.by(() => {
    const data = $actionsQuery.data ?? [];
    const query = search.trim();

    if (query.length < 1) {
      return data;
    }

    return filterCategories(data, query);
  });

  function filterCategories(categories: ActionCategory[], query: string) {
    query = query.toLowerCase();

    return categories
      .map((category) => {
        return {
          ...category,

          actions: category.actions.filter((action) => {
            const name = action.label.toLowerCase();
            return name === query || name.includes(query);
          }),
        };
      })
      .filter((pack) => pack.actions.length > 0);
  }

  function isCategoryExpanded(pluginId: PluginId): boolean {
    const expandedCategories = actionSidebarState.current.expandedCategories;
    if (!expandedCategories) return true;

    return expandedCategories[pluginId] ?? true;
  }

  function onToggleCategoryExpanded(pluginId: PluginId) {
    actionSidebarState.current = {
      ...actionSidebarState.current,
      expandedCategories: {
        ...actionSidebarState.current.expandedCategories,
        [pluginId]: !isCategoryExpanded(pluginId),
      },
    };
  }

  function onToggleExpanded() {
    actionSidebarState.current = {
      ...actionSidebarState.current,
      expanded: !(actionSidebarState.current.expanded ?? true),
    };
  }
</script>

<div class="sidebar" class:sidebar--expanded={sidebarExpanded}>
  <button class="sidebar-expand" onclick={onToggleExpanded}>
    {#if sidebarExpanded}
      <SolarAltArrowRightOutline />
    {:else}
      <SolarAltArrowLeftOutline />
    {/if}
  </button>
  <div class="sidebar-header">
    <div class="search-wrapper">
      <input
        bind:value={search}
        class="search"
        type="text"
        placeholder="Search..."
      />
    </div>
  </div>

  <div class="content">
    {#if $actionsQuery.isLoading}
      Loading actions...
    {:else if $actionsQuery.isError}
      Failed to load actions: {getErrorMessage($actionsQuery.error)}
    {:else if $actionsQuery.isSuccess}
      {#each filteredCategories as category}
        <ActionsSidebarCategory
          {category}
          expanded={isCategoryExpanded(category.plugin_id)}
          onToggleExpanded={() => onToggleCategoryExpanded(category.plugin_id)}
        />
      {:else}
        <p class="none">No results found...</p>
      {/each}
    {/if}
  </div>
</div>

<style>
  .sidebar {
    position: relative;
    flex-shrink: 0;
    background-color: #131316;
    height: 100%;
    border-left: 1px solid #333;
    display: flex;
    flex-flow: column;
    width: 1rem;
    transition: 0.25s ease width;
  }

  .sidebar-header {
    background-color: #1a191d;
  }

  .sidebar-expand {
    display: flex;
    align-items: center;
    justify-content: center;
    color: #fff;
    position: absolute;
    left: -2rem;
    top: 0rem;
    z-index: 20;
    width: 2rem;
    height: 2rem;
    background-color: #1a191d;
    border-bottom-left-radius: 0.25rem;
    border: none;

    border-left: 1px solid #333;
    border-bottom: 1px solid #333;
    cursor: pointer;
  }

  .sidebar--expanded {
    width: 15rem;
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
    width: 100%;
  }

  .search-wrapper {
    padding: 0.5rem;
  }

  .none {
    padding: 0.5rem;
  }

  .content {
    overflow-x: hidden;
    overflow-y: auto;
    width: 15rem;
  }
</style>
