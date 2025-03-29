<script lang="ts">
  import type { Icon, IconPack } from "$lib/api/types/icons";

  import { onMount } from "svelte";
  import { passiveEventArg } from "$lib/utils/browser";
  import { getIconAssetPath } from "$lib/api/utils/url";

  type Props = {
    pack: IconPack;
    // Available items for the grid
    items: Icon[];
    itemHeight: number;
    columns: number;
    onClickIcon: (icon: Icon) => void;
  };

  const { pack, items, itemHeight, columns, onClickIcon }: Props = $props();

  let containerHeight: number = $state(0);
  let wrapper: HTMLDivElement | undefined = $state();
  let wrapperStyle = $state("");

  const rowCount = $derived(Math.ceil(items.length / columns));
  const totalSize = $derived(rowCount * itemHeight);

  // Current scroll offset
  let offset = $state(0);

  const virtualItems: VirtualItems = $derived.by(() =>
    createVirtualItems(containerHeight, offset, rowCount, itemHeight),
  );

  $effect(() => {
    console.log(virtualItems.startIndex, "Virtual items updated");
  });

  type VirtualItems = {
    startIndex: number;
    stopIndex: number;
  };

  onMount(() => {
    if (wrapper === undefined) return;

    const resizeObserver = new ResizeObserver((entries) => {
      for (let entry of entries) {
        containerHeight = entry.target.clientHeight;
      }
    });

    containerHeight = wrapper.clientHeight;

    resizeObserver.observe(wrapper);
    wrapper.addEventListener("scroll", handleScroll, passiveEventArg);

    return () => {
      if (wrapper === undefined) return;
      resizeObserver.disconnect();
      wrapper.removeEventListener("scroll", handleScroll);
    };
  });

  function binarySearch({
    low,
    high,
    offset,
  }: {
    low: number;
    high: number;
    offset: number;
  }) {
    let middle = 0;
    let currentOffset = 0;

    while (low <= high) {
      middle = low + Math.floor((high - low) / 2);
      currentOffset = middle * itemHeight;

      if (currentOffset === offset) {
        return middle;
      } else if (currentOffset < offset) {
        low = middle + 1;
      } else if (currentOffset > offset) {
        high = middle - 1;
      }
    }

    if (low > 0) {
      return low - 1;
    }

    return 0;
  }

  function createVirtualItems(
    containerSize: number,
    offset: number,
    rowCount: number,
    itemHeight: number,
  ): VirtualItems {
    const overscanCount = 2;
    const maxOffset = offset + containerSize;

    const totalSize = rowCount * itemHeight;

    if (totalSize === 0) {
      return {
        startIndex: 0,
        stopIndex: 0,
      };
    }

    // Find the first visible row
    const startRow = Math.max(
      0,
      binarySearch({
        high: rowCount,
        low: 0,
        offset: Math.max(0, offset),
      }) - overscanCount,
    );

    const startOffset = startRow * itemHeight;
    const visibleHeight = maxOffset - startOffset;
    const visibleRows = Math.ceil(visibleHeight / itemHeight);

    // Determine the last visible row
    const stopRow = Math.min(
      startRow + visibleRows + overscanCount,
      rowCount - 1,
    );

    const startIndex: number = startRow * columns;
    const stopIndex: number = stopRow * columns + columns - 1;

    return {
      startIndex,
      stopIndex,
    };
  }

  function getItemStyle(index: number) {
    // Compute row
    const row = Math.floor(index / columns);

    // Compute column
    const column = index % columns;

    const offset = row * itemHeight;
    const width = (1 / columns) * 100;
    const left = (1 / columns) * column * 100;
    const style = `height:${itemHeight}px;position:absolute;top:${offset}px;width:${width}%;left:${left}%;`;

    return style;
  }

  function handleScroll(event: Event) {
    requestAnimationFrame(() => {
      const scrollOffset = wrapper?.scrollTop ?? 0;

      if (
        scrollOffset < 0 ||
        offset === scrollOffset ||
        event.target !== wrapper
      )
        return;

      offset = scrollOffset;
    });
  }
</script>

<div bind:this={wrapper} class="wrapper" style={wrapperStyle}>
  <div style={`height:${totalSize}px;`} class="grid">
    {#each items.slice(virtualItems.startIndex, virtualItems.stopIndex + 1) as icon, index (virtualItems.startIndex + index)}
      <button
        style={getItemStyle(virtualItems.startIndex + index)}
        class="item-wrapper"
        onclick={() => onClickIcon(icon)}
      >
        <img
          src={getIconAssetPath(pack.manifest.icons.id, icon.path)}
          alt={icon.name}
          loading="lazy"
          width="64px"
          height="64px"
        />
      </button>
    {/each}
  </div>
</div>

<style>
  .item-wrapper {
    position: relative;
  }

  .wrapper {
    overflow: auto;
    will-change: transform;
    -webkit-overflow-scrolling: touch;
    height: 100%;
    width: 100%;
    position: relative;
  }
</style>
