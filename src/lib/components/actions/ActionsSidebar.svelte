<script lang="ts">
  import type { PluginId } from "$lib/api/types/plugin";

  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { persistedState } from "$lib/utils/localStorage.svelte";
  import SolarAltArrowLeftOutline from "~icons/solar/alt-arrow-left-outline";
  import SolarAltArrowRightOutline from "~icons/solar/alt-arrow-right-outline";

  import Aside from "../Aside.svelte";
  import SkeletonList from "../skeleton/SkeletonList.svelte";
  import ActionsSidebarCategory, {
    type ActionCategoryData,
  } from "./ActionCategory.svelte";

  type Props = {
    actions: ActionCategoryData[];
    actionsLoading: boolean;
    actionsError: Error | null;
  };

  type ActionSidebarState = {
    expanded?: boolean;
    expandedCategories?: Partial<Record<PluginId, boolean>>;
  };

  const { actions, actionsError, actionsLoading }: Props = $props();

  const i18n = i18nContext.get();

  const actionSidebarState = persistedState<ActionSidebarState>(
    "actionSidebarState",
    { expanded: true, expandedCategories: {} },
  );

  const sidebarExpanded = $derived(actionSidebarState.current.expanded ?? true);

  let search = $state("");

  const filteredCategories = $derived.by(() => {
    const query = search.trim();

    if (query.length < 1) {
      return actions;
    }

    return filterCategories(actions, query);
  });

  function filterCategories(categories: ActionCategoryData[], query: string) {
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
        placeholder={i18n.f("search_placeholder")}
      />
    </div>
  </div>

  <div class="content">
    {#if actionsLoading}
      <SkeletonList style="margin: 1rem" />
    {:else if actionsError}
      <Aside severity="error" style="margin: 1rem;">
        {i18n.f("actions_error", {
          values: { error: getErrorMessage(actionsError) },
        })}
      </Aside>
    {:else}
      {#each filteredCategories as category (category.pluginId)}
        <ActionsSidebarCategory
          pluginId={category.pluginId}
          icon={category.icon}
          label={category.label}
          actions={category.actions}
          expanded={isCategoryExpanded(category.pluginId)}
          onToggleExpanded={() => onToggleCategoryExpanded(category.pluginId)}
        />
      {:else}
        <p class="none">
          {i18n.f("no_results")}
        </p>
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
