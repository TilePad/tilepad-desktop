<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";

  import EmptyTile from "./EmptyTile.svelte";
  import FilledTile from "./FilledTile.svelte";

  type Props = {
    tiles: TileModel[];
    rows: number;
    columns: number;
    onClickTile: (tile: TileModel) => void;
  };

  const { tiles, rows, columns, onClickTile }: Props = $props();

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

  const items = $derived(createGridItems(tiles));

  function createGridItems(tiles: TileModel[]) {
    const out = [];
    for (let i = 0; i < rows * columns; i += 1) {
      const row = Math.floor(i / columns);
      const column = i % columns;
      const tile = getTile(tiles, row, column);
      const id = tile?.id ?? `${i}`;

      out.push({ id, tile, row, column });
    }
    return out;
  }

  function getTile(tiles: TileModel[], row: number, column: number) {
    return tiles.find((tile) => tile.row === row && tile.column === column);
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
    {@const tile = item.tile}
    {#if tile !== undefined}
      <FilledTile {tile} onClick={() => onClickTile(tile)} />
    {:else}
      <EmptyTile row={item.row} column={item.column} />
    {/if}
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
