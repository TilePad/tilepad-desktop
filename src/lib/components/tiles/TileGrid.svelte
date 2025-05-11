<script lang="ts" module>
  export const DESIRED_TILE_WIDTH = 120;
</script>

<script lang="ts">
  import type { TileId, TileModel } from "$lib/api/types/tiles";

  import EmptyTile from "./EmptyTile.svelte";
  import FilledTile from "./FilledTile.svelte";

  type Props = {
    tiles: TileModel[];
    rows: number;
    columns: number;
    onClickTile: (tile: TileModel) => void;
  };

  const { tiles, rows, columns, onClickTile }: Props = $props();

  let containerWidth = $state(0);
  let containerHeight = $state(0);

  const gap = 10;

  const { tileSize, left, width, height } = $derived.by(() => {
    const tileSize = Math.min(
      (containerWidth - gap * (columns - 1)) / columns,
      (containerHeight - gap * (rows - 1)) / rows,
    );

    const spannedWidth = tileSize * columns + gap * (columns - 1);
    const spannedHeight = tileSize * rows + gap * (rows - 1);

    const left = (containerWidth - spannedWidth) / 2;

    return {
      tileSize,
      width: spannedWidth,
      height: spannedHeight,
      left,
    };
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

  function isTileWithin(
    col: number,
    colSpan: number,
    row: number,
    rowSpan: number,
    exclude: TileId,
  ): boolean {
    const colEnd = col + colSpan;
    const rowEnd = row + rowSpan;

    for (const tile of tiles) {
      if (
        tile.id !== exclude &&
        tile.row >= row &&
        tile.column >= col &&
        tile.row < rowEnd &&
        tile.column < colEnd
      ) {
        return true;
      }
    }

    return false;
  }
</script>

<div
  class="container"
  bind:clientWidth={containerWidth}
  bind:clientHeight={containerHeight}
>
  <div
    class="grid"
    style="transform: translateX({left}px); width: {width}px; height: {height}px;"
  >
    {#each items as item}
      {@const tile = item.tile}
      {#if tile !== undefined}
        <FilledTile
          {tile}
          {tileSize}
          {gap}
          onClick={() => onClickTile(tile)}
          {isTileWithin}
        />
      {:else}
        <EmptyTile row={item.row} column={item.column} width={tileSize} {gap} />
      {/if}
    {/each}
  </div>
</div>

<style>
  .grid {
    position: relative;
  }

  .container {
    width: 100%;
    height: 100%;
  }
</style>
