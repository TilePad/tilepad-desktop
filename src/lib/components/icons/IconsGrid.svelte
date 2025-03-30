<script lang="ts">
  import type { Icon, IconPack } from "$lib/api/types/icons";

  import { range } from "$lib/api/utils/svelte.svelte";
  import { getIconAssetPath } from "$lib/api/utils/url";
  import { createVirtualizer } from "$lib/utils/virtualization.svelte";

  type Props = {
    pack: IconPack;
    // Available items for the grid
    items: Icon[];
    itemHeight: number;
    columns: number;
    onClickIcon: (icon: Icon) => void;
  };

  const { pack, items, itemHeight, columns, onClickIcon }: Props = $props();

  let wrapper: HTMLDivElement | undefined = $state();

  const rowCount = $derived(Math.ceil(items.length / columns));

  const { totalHeight, startRow, stopRow } = $derived.by(
    createVirtualizer(
      () => wrapper,
      () => rowCount,
      () => itemHeight,
    ),
  );

  // Compute width of each column (Percent)
  const columnWidth = $derived((1 / columns) * 100);
</script>

<div bind:this={wrapper} class="scroll-container">
  <div style={`height:${totalHeight}px;`}>
    {#each range(startRow, stopRow + 1) as row (row)}
      <!-- Compute row offset -->
      {@const rowOffset = row * itemHeight}

      {#each range(0, columns) as col (col)}
        <!-- Get the item at the position -->
        {@const icon = items[row * columns + col]}

        {#if icon !== undefined}
          <!-- Compute column offset -->
          {@const columnOffset = col * columnWidth}

          <button
            class="item"
            style="position:absolute;top:0px;left:{columnOffset}%;width:{columnWidth}%;height:{itemHeight}px;transform:translateY({rowOffset}px)"
            onclick={() => onClickIcon(icon)}
          >
            <img
              src={getIconAssetPath(pack.manifest.icons.id, icon.path)}
              alt={icon.name}
              width="64px"
              height="64px"
              loading="lazy"
            />
          </button>
        {/if}
      {/each}
    {/each}
  </div>
</div>

<style>
  .scroll-container {
    overflow: auto;
    -webkit-overflow-scrolling: touch;
    height: 100%;
    width: 100%;
    position: relative;
  }

  .item {
    will-change: transform;
  }
</style>
