<script lang="ts">
  import type { ActionCategory } from "$lib/api/types/actions";

  import { getPluginAssetPath } from "$lib/api/utils/url";
  import SolarAltArrowDownOutline from "~icons/solar/alt-arrow-down-outline";
  import SolarAltArrowRightOutline from "~icons/solar/alt-arrow-right-outline";

  import ActionsSidebarList from "./ActionList.svelte";

  type Props = {
    category: ActionCategory;
  };

  const { category }: Props = $props();

  let expanded = $state(true);

  const onToggleCollapsed = () => (expanded = !expanded);
</script>

<div class="section">
  <button class="header" onclick={onToggleCollapsed}>
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
    <div class="content">
      <ActionsSidebarList actions={category.actions} />
    </div>
  {/if}
</div>

<style>
  .header {
    display: flex;
    align-items: center;
    background-color: #3d3844;
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
