<script lang="ts" module>
  export interface IconGridItem {
    name: string;
    path: string;
    src: string;
  }
</script>

<script lang="ts">
  import { createVirtualizer } from "$lib/utils/virtualization.svelte";

  type Props = {
    /** Available items for the grid */
    items: IconGridItem[];
    itemHeight: number;
    columns: number;
    onClickIcon: (icon: IconGridItem) => void;
  };

  type VirtualItem = {
    index: number;
    icon: IconGridItem;
    style: string;
  };

  const { items, itemHeight, columns, onClickIcon }: Props = $props();

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

  let styleCache: Record<number, string> = {};

  function getRowStyle(index: number) {
    const cached = styleCache[index];
    if (cached) return cached;

    const row = Math.floor(index / columns);
    const column = Math.floor(index % columns);
    const rowOffset = row * itemHeight;
    const columnOffset = column * columnWidth;

    const style = `left:${columnOffset}%;width:${columnWidth}%;height:${itemHeight}px;transform:translateY(${rowOffset}px)`;
    styleCache[index] = style;
    return style;
  }

  const renderItems = $derived.by(() => {
    let renderItems: VirtualItem[] = [];

    for (
      let index = startRow * columns;
      index < (stopRow + 1) * columns;
      index += 1
    ) {
      const icon = items[index];
      if (icon === undefined) break;

      renderItems.push({
        index,
        icon,
        style: getRowStyle(index),
      });
    }

    return renderItems;
  });
</script>

<div bind:this={wrapper} class="scroll-container">
  <div style={`height:${totalHeight}px;`}>
    {#each renderItems as item (item.index)}
      <button
        class="item"
        style={item.style}
        onclick={() => onClickIcon(item.icon)}
      >
        <img
          class="item__image"
          src={item.icon.src}
          alt={item.icon.name}
          width="64px"
          height="64px"
        />
      </button>
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
    contain: strict;
    content-visibility: auto;
  }

  .item {
    will-change: transform;
    background-color: transparent;
    border: none;
    cursor: pointer;

    position: absolute;
    top: 0px;
  }

  .item__image {
    width: 64px;
    height: 64px;
  }
</style>
