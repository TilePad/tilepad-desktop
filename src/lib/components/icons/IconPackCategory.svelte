<script lang="ts">
  import type { Icon, IconPack } from "$lib/api/types/icons";

  import SolarAltArrowDownOutline from "~icons/solar/alt-arrow-down-outline";
  import SolarAltArrowRightOutline from "~icons/solar/alt-arrow-right-outline";

  import IconsGrid from "./IconsGrid.svelte";

  type Props = {
    pack: IconPack;
    onClickIcon: (icon: Icon) => void;
  };

  const { pack, onClickIcon }: Props = $props();

  let expanded = $state(false);

  const onToggleCollapsed = () => (expanded = !expanded);
</script>

<div class="section">
  <button class="header" onclick={onToggleCollapsed}>
    {#if expanded}
      <SolarAltArrowDownOutline />
    {:else}
      <SolarAltArrowRightOutline />
    {/if}

    {pack.manifest.icons.name}
    {pack.icons.length}
  </button>

  {#if expanded}
    <div class="content">
      <IconsGrid
        {onClickIcon}
        {pack}
        items={pack.icons}
        columns={6}
        itemHeight={64}
      />
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
