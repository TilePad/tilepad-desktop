<script lang="ts">
  import type { Icon } from "$lib/api/types/icons";

  import ArrowDownIcon from "~icons/solar/alt-arrow-down-outline";
  import ArrowRightIcon from "~icons/solar/alt-arrow-right-outline";

  import IconsGrid, { type IconGridItem } from "./IconsGrid.svelte";

  type Props = {
    name: string;
    icons: IconGridItem[];

    onClickIcon: (icon: Icon) => void;
  };

  const { name, icons, onClickIcon }: Props = $props();

  let expanded = $state(false);

  const onToggleCollapsed = () => (expanded = !expanded);
</script>

<div class="section">
  <button class="header" onclick={onToggleCollapsed}>
    {#if expanded}
      <ArrowDownIcon />
    {:else}
      <ArrowRightIcon />
    {/if}

    {name}
    {icons.length}
  </button>

  {#if expanded}
    <div class="content">
      <IconsGrid {onClickIcon} items={icons} columns={6} itemHeight={64} />
    </div>
  {/if}
</div>

<style>
  .content {
    height: 15rem;
    width: 100%;
    overflow: hidden;
  }

  .header {
    width: 100%;
    display: flex;
    align-items: center;
    background-color: #3d3844;
    border: none;
    padding: 0.5rem;
    color: #fff;
    gap: 0.5rem;
    cursor: pointer;
  }
</style>
