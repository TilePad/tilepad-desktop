<script lang="ts">
  import type { PluginId } from "$lib/api/types/plugin";
  import type { ActionCategory } from "$lib/api/types/actions";

  import { createActionsQuery } from "$lib/api/actions";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { persistedState } from "$lib/utils/localStorage.svelte";

  import ActionsSidebarCategory from "./ActionCategory.svelte";

  type ExpandedCategories = Partial<Record<PluginId, boolean>>;

  const actionsQuery = createActionsQuery();
  const expandedCategories = persistedState<ExpandedCategories>(
    "actionsExpandedCategories",
    {},
  );

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
    return expandedCategories.current[pluginId] ?? true;
  }

  function onToggleCategoryExpanded(pluginId: PluginId) {
    expandedCategories.current = {
      ...expandedCategories.current,
      [pluginId]: !isCategoryExpanded(pluginId),
    };
  }
</script>

<div class="sidebar">
  <div class="search-wrapper">
    <input
      bind:value={search}
      class="search"
      type="text"
      placeholder="Search..."
    />
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
    flex-shrink: 0;
    width: 15rem;
    background-color: #29262e;
    height: 100%;
    border-left: 1px solid #333;
    overflow: hidden;
    display: flex;
    flex-flow: column;
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
  }
</style>
