<script lang="ts" module>
  import type { PluginId } from "$lib/api/types/plugin";

  import type { ActionItemData } from "./ActionItem.svelte";

  export interface ActionCategoryData {
    pluginId: PluginId;

    label: string;
    icon?: string;
    actions: ActionItemData[];
  }
</script>

<script lang="ts">
  import { slide } from "svelte/transition";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import SolarAltArrowDownOutline from "~icons/solar/alt-arrow-down-outline";
  import SolarAltArrowRightOutline from "~icons/solar/alt-arrow-right-outline";

  import ActionsSidebarList from "./ActionList.svelte";

  type Props = ActionCategoryData & {
    expanded: boolean;
    onToggleExpanded: VoidFunction;
  };

  const { label, icon, actions, expanded, onToggleExpanded }: Props = $props();

  const i18n = i18nContext.get();
</script>

<div class="section">
  <button class="header" onclick={onToggleExpanded}>
    {#if expanded}
      <SolarAltArrowDownOutline />
    {:else}
      <SolarAltArrowRightOutline />
    {/if}

    {#if icon}
      <img class="icon" src={icon} alt={i18n.f("action_icon")} />
    {/if}

    {label}
  </button>

  {#if expanded}
    <div
      class="content"
      style="height: {40 * actions.length}px;"
      transition:slide={{ axis: "y", duration: 200 }}
    >
      <ActionsSidebarList {actions} />
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
