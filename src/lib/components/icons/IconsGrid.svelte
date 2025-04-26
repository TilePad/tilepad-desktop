<script lang="ts">
  import type { Icon, IconPack } from "$lib/api/types/icons";

  import { getIconAssetPath } from "$lib/api/utils/url";
  import { createVirtualizer } from "$lib/utils/virtualization.svelte";

  import { getServerContext } from "../ServerProvider.svelte";

  type Props = {
    pack: IconPack;
    // Available items for the grid
    items: Icon[];
    itemHeight: number;
    columns: number;
    onClickIcon: (icon: Icon) => void;
  };

  const { pack, items, itemHeight, columns, onClickIcon }: Props = $props();

  const serverContext = getServerContext();
  const serverURL = $derived(serverContext.serverURL);

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

  function getRowStyle(index: number) {
    const row = Math.floor(index / columns);
    const column = Math.floor(index % columns);
    const rowOffset = row * itemHeight;
    const columnOffset = column * columnWidth;

    return `left:${columnOffset}%;width:${columnWidth}%;height:${itemHeight}px;transform:translateY(${rowOffset}px)`;
  }

  type VirtualItem = {
    index: number;
    icon: Icon;
    style: string;
  };

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
          src={getIconAssetPath(
            serverURL,
            pack.manifest.icons.id,
            item.icon.path,
          )}
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
