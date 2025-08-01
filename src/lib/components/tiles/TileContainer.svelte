<script lang="ts">
  import type { Snippet } from "svelte";
  import type { TilePosition } from "$lib/api/types/tiles";

  import { DESIRED_TILE_WIDTH } from "./TileGrid.svelte";

  type Props = {
    position: TilePosition;
    tileSize: number;
    gap: number;
    resizing?: boolean;
    empty?: boolean;
    children?: Snippet;
  };

  const { position, tileSize, gap, resizing, empty, children }: Props =
    $props();

  const { tileX, tileY, tileZ, tileWidth, tileHeight, sizeAdjust } =
    $derived.by(() => {
      const tileWidth =
        tileSize * position.column_span + gap * (position.column_span - 1);
      const tileHeight =
        tileSize * position.row_span + gap * (position.row_span - 1);

      const ratioX = (tileWidth - DESIRED_TILE_WIDTH) / DESIRED_TILE_WIDTH;
      const ratioY = (tileHeight - DESIRED_TILE_WIDTH) / DESIRED_TILE_WIDTH;

      const sizeAdjustX = 1 + ratioX;
      const sizeAdjustY = 1 + ratioY;
      const sizeAdjust = Math.min(sizeAdjustX, sizeAdjustY);

      const tileX = tileSize * position.column + gap * position.column;
      const tileY = tileSize * position.row + gap * position.row;
      const tileZ = empty
        ? 0
        : (position.row + position.row_span) *
          (position.column + position.column_span);

      return {
        tileX,
        tileY,
        tileZ,
        tileWidth,
        tileHeight,
        sizeAdjust,
      };
    });
</script>

<div
  class:tile-container--resizing={resizing}
  class="tile-container"
  style="--tile-size-adjustment: {sizeAdjust}; --tile-width: {tileWidth}px; --tile-height: {tileHeight}px; --tile-x: {tileX}px; --tile-y: {tileY}px; --tile-z: {tileZ}"
>
  {@render children?.()}
</div>

<style>
  .tile-container {
    position: absolute;
    top: 0;
    left: 0;

    transform: translate(var(--tile-x), var(--tile-y));
    width: var(--tile-width);
    height: var(--tile-height);
    z-index: calc(var(--tile-z));
  }

  .tile-container--resizing {
    transition: all 0.1s ease;
  }
</style>
