<script lang="ts">
  import { getDraggingContext } from "./TileDraggingProvider.svelte";

  type Props = {
    row: number;
    column: number;
    width: number;
    gap: number;
  };
  const { dropZoneTarget } = getDraggingContext();

  const { row, column, width, gap }: Props = $props();

  const { tileX, tileY, tileWidth, tileHeight } = $derived.by(() => {
    const tileWidth = width;
    const tileHeight = width;

    const tileX = tileWidth * column + gap * column;
    const tileY = tileHeight * row + gap * row;

    return {
      tileX,
      tileY,
      tileWidth,
      tileHeight,
    };
  });

  const dropping = $derived.by(() => {
    const target = dropZoneTarget();
    if (target === null) return false;
    return (
      target.type === "emptyTile" &&
      target.row === row &&
      target.column === column
    );
  });
</script>

<div
  class="tile-container"
  style="--tile-width: {tileWidth}px; --tile-height: {tileHeight}px; --tile-x: {tileX}px; --tile-y: {tileY}px;"
>
  <div
    class="tile"
    class:tile--dropping={dropping}
    data-drop-zone="emptyTile"
    data-row={row}
    data-column={column}
  ></div>
</div>

<style>
  .tile-container {
    position: absolute;
    top: 0;
    left: 0;

    transform: translate(var(--tile-x), var(--tile-y));
    width: var(--tile-width);
    height: var(--tile-height);
  }

  .tile {
    position: relative;
    background-color: #242129;
    border: 2px solid #36313d;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    width: var(--tile-width);
    height: var(--tile-height);
    color: #ccc;

    user-select: none;
    overflow: hidden;

    font-size: 1.5rem;
    text-align: center;
    z-index: 0;
  }

  .tile--dropping {
    background-color: #3a3542;
    border-color: #36313d;
  }
</style>
