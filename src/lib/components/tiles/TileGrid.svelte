<script lang="ts">
  import type { Snippet } from "svelte";

  type Props = {
    rows: number;
    columns: number;
    tile: Snippet<[number, number]>;
  };

  const { rows, columns, tile }: Props = $props();

  let container: HTMLDivElement | undefined = $state();

  let containerWidth = $state(0);
  let containerHeight = $state(0);

  const gap = 10;

  const desiredWidth = 120;

  const tileWidth = $derived(
    Math.min(
      (containerWidth - gap * (columns - 1)) / columns,
      (containerHeight - gap * (rows - 1)) / rows,
    ),
  );

  const sizeAdjust = $derived.by(() => {
    const ratio = (tileWidth - desiredWidth) / desiredWidth;
    return 1 + ratio;
  });

  const items = $derived(createGridItems());

  function createGridItems() {
    const out = [];
    for (let i = 0; i < rows * columns; i += 1) {
      const row = Math.floor(i / columns);
      const column = i % columns;
      out.push({ id: i, row, column });
    }
    return out;
  }
</script>

<div
  class="grid"
  style="--tile-size-adjustment: {sizeAdjust}; --tile-width: {tileWidth}px; --rows: {rows}; --columns: {columns}; --tile-gap: {gap}px;"
  bind:this={container}
  bind:clientWidth={containerWidth}
  bind:clientHeight={containerHeight}
>
  {#each items as item}
    {@render tile(item.row, item.column)}
  {/each}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(var(--columns), var(--tile-width));
    grid-auto-rows: var(--tile-width);
    justify-content: center;

    width: 100%;
    height: 100%;
    gap: var(--tile-gap);
  }
</style>
