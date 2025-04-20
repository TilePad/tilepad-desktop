<script lang="ts">
  import type { ActionCategory } from "$lib/api/types/actions";

  import { slide } from "svelte/transition";
  import { getPluginAssetPath } from "$lib/api/utils/url";
  import SolarAltArrowDownOutline from "~icons/solar/alt-arrow-down-outline";
  import SolarAltArrowRightOutline from "~icons/solar/alt-arrow-right-outline";

  import ActionsSidebarList from "./ActionList.svelte";

  type Props = {
    category: ActionCategory;
    expanded: boolean;
    onToggleExpanded: VoidFunction;
  };

  const { category, expanded, onToggleExpanded }: Props = $props();
</script>

<div class="section">
  <button class="header" onclick={onToggleExpanded}>
    {#if expanded}
      <SolarAltArrowDownOutline />
    {:else}
      <SolarAltArrowRightOutline />
    {/if}

    {#if category.icon !== null}
      <img
        class="icon"
        src={getPluginAssetPath(category.plugin_id, category.icon)}
        alt="Action Icon"
      />
    {/if}

    {category.label}
  </button>

  {#if expanded}
    <div
      class="content"
      style="height: {40 * category.actions.length}px;"
      transition:slide={{ axis: "y", duration: 200 }}
    >
      <ActionsSidebarList actions={category.actions} />
    </div>
  {/if}
</div>

<style>
  .header {
    display: flex;
    align-items: center;
    background-color: #1f1e22;
    border: none;
    width: 100%;
    padding: 0.5rem;
    color: #fff;
    gap: 0.5rem;
    cursor: pointer;
  }

  .content {
    flex: auto;
    width: 15rem;
  }

  .section {
    width: 15rem;
  }

  .icon {
    max-width: 1rem;
  }
</style>
