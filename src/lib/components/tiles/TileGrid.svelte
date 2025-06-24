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
    return tiles.find(
      (tile) => tile.position.row === row && tile.position.column === column,
    );
  }

  function isAllowedWithin(
    col: number,
    colSpan: number,
    row: number,
    rowSpan: number,
    exclude: TileId,
  ): boolean {
    const colEnd = col + colSpan;
    const rowEnd = row + rowSpan;

    // Out of valid bounds
    if (col < 0 || row < 0) {
      return false;
    }

    // Out of bounds for a valid tile
    if (col > columns || colEnd > columns || row > rows || rowEnd > rows) {
      return false;
    }

    for (const tile of tiles) {
      const { position } = tile;

      const tileRowEnd = position.row + position.row_span;
      const tileColEnd = position.column + position.column_span;

      if (
        tile.id !== exclude &&
        position.row >= row &&
        position.column >= col &&
        tileRowEnd <= rowEnd &&
        tileColEnd <= colEnd
      ) {
        return false;
      }
    }

    return true;
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
    {#each items as item (item.id)}
      {@const tile = item.tile}
      {#if tile !== undefined}
        <FilledTile
          {tile}
          {tileSize}
          {gap}
          onClick={() => onClickTile(tile)}
          {isAllowedWithin}
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
