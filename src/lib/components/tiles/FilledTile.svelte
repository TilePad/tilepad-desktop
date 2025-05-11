<script lang="ts">
  import type { TileId, TileModel } from "$lib/api/types/tiles";

  import {
    resizeHandle,
    ResizeDirection,
    type ResizeEventDetail,
  } from "$lib/utils/resizable";

  import TileIcon from "./TileIcon.svelte";
  import TileLabelElm from "./TileLabelElm.svelte";
  import { DESIRED_TILE_WIDTH } from "./TileGrid.svelte";
  import { getDraggingContext } from "./TileDraggingProvider.svelte";

  type Props = {
    tile: TileModel;
    tileSize: number;
    gap: number;

    onClick: VoidFunction;

    isTileWithin: (
      col: number,
      colSpan: number,
      row: number,
      rowSpan: number,
      exclude: TileId,
    ) => boolean;
  };

  const { tile, tileSize, gap, onClick, isTileWithin }: Props = $props();
  const { draggingState, onStartDragging } = getDraggingContext();

  let lastRowSpan: number = $state(1);
  let lastColSpan: number = $state(1);

  let lastColStart = $state(tile.column);
  let lastRowStart = $state(tile.row);

  let colStart = $state(tile.column);
  let rowStart = $state(tile.row);

  let colSpan = $state(1);
  let rowSpan = $state(1);

  const { tileX, tileY, tileZ, tileWidth, tileHeight, sizeAdjust } =
    $derived.by(() => {
      const tileWidth = tileSize * colSpan + gap * (colSpan - 1);
      const tileHeight = tileSize * rowSpan + gap * (rowSpan - 1);

      const ratioX = (tileWidth - DESIRED_TILE_WIDTH) / DESIRED_TILE_WIDTH;
      const ratioY = (tileHeight - DESIRED_TILE_WIDTH) / DESIRED_TILE_WIDTH;

      const sizeAdjustX = 1 + ratioX;
      const sizeAdjustY = 1 + ratioY;
      const sizeAdjust = Math.min(sizeAdjustX, sizeAdjustY);

      const tileX = tileSize * colStart + gap * colStart;
      const tileY = tileSize * rowStart + gap * rowStart;
      const tileZ = colStart + colSpan + rowStart + rowSpan;

      return {
        tileX,
        tileY,
        tileZ,
        tileWidth,
        tileHeight,
        sizeAdjust,
      };
    });

  let touchTimeout: number | undefined;
  let button: HTMLButtonElement | undefined = $state();

  const config = $derived(tile.config);
  const dragging = $derived.by(() => {
    const target = draggingState();
    if (target === null) return false;
    return target.data.type === "tile" && target.data.id === tile.id;
  });

  function onPointerDown(event: PointerEvent) {
    if (touchTimeout) {
      clearTimeout(touchTimeout);
      touchTimeout = undefined;
    }

    touchTimeout = setTimeout(() => {
      if (!button) return;
      touchTimeout = undefined;
      onStartDragging(event, { type: "tile", ...tile }, button);
    }, 100);
  }

  function onPointerUp() {
    if (touchTimeout) {
      clearTimeout(touchTimeout);
      touchTimeout = undefined;
    }
  }

  function handleResizeVerticalTop(event: CustomEvent<ResizeEventDetail>) {
    const newStart =
      lastRowStart + Math.min(event.detail.scaleY, lastRowSpan - 1);
    const newSpan = Math.max(lastRowSpan - event.detail.scaleY, 1);

    // Prevent overlapping
    if (isTileWithin(colStart, colSpan, newStart, newSpan, tile.id)) {
      return;
    }

    rowStart = newStart;
    rowSpan = newSpan;

    if (event.detail.commit) {
      lastRowStart = rowStart;
      lastRowSpan = rowSpan;

      // TODO: Persist
    }
  }

  function handleResizeVerticalBottom(event: CustomEvent<ResizeEventDetail>) {
    const newSpan = Math.max(lastRowSpan + event.detail.scaleY, 1);

    // Prevent overlapping
    if (isTileWithin(colStart, colSpan, rowStart, newSpan, tile.id)) {
      return;
    }

    rowSpan = newSpan;

    if (event.detail.commit) {
      lastRowSpan = rowSpan;

      // TODO: Persist
    }
  }

  function handleResizeHorizontalLeft(event: CustomEvent<ResizeEventDetail>) {
    const newStart =
      lastColStart + Math.min(event.detail.scaleX, lastColSpan - 1);
    const newSpan = Math.max(lastColSpan - event.detail.scaleX, 1);

    // Prevent overlapping
    if (isTileWithin(newStart, newSpan, rowStart, rowSpan, tile.id)) {
      return;
    }

    colStart = newStart;
    colSpan = newSpan;

    if (event.detail.commit) {
      lastColSpan = colSpan;
      lastColStart = colStart;

      // TODO: Persist
    }
  }

  function handleResizeHorizontalRight(event: CustomEvent<ResizeEventDetail>) {
    const newSpan = Math.max(lastColSpan + event.detail.scaleX, 1);

    // Prevent overlapping
    if (isTileWithin(colStart, newSpan, rowStart, rowSpan, tile.id)) {
      return;
    }

    colSpan = newSpan;

    if (event.detail.commit) {
      lastColSpan = colSpan;

      // TODO: Persist
    }
  }

  function considerResizeDiagonalLeftTop(
    event: CustomEvent<ResizeEventDetail>,
  ) {
    handleResizeHorizontalLeft(event);
    handleResizeVerticalTop(event);
  }

  function considerResizeDiagonalLeftBottom(
    event: CustomEvent<ResizeEventDetail>,
  ) {
    handleResizeHorizontalLeft(event);
    handleResizeVerticalBottom(event);
  }

  function considerResizeDiagonalRightBottom(
    event: CustomEvent<ResizeEventDetail>,
  ) {
    handleResizeHorizontalRight(event);
    handleResizeVerticalBottom(event);
  }

  function considerResizeDiagonalRightTop(
    event: CustomEvent<ResizeEventDetail>,
  ) {
    handleResizeHorizontalRight(event);
    handleResizeVerticalTop(event);
  }

  const distanceThreshold = $derived(tileSize + gap);
</script>

<div
  class="tile-container"
  style="--tile-size-adjustment: {sizeAdjust}; --tile-width: {tileWidth}px; --tile-height: {tileHeight}px; --tile-x: {tileX}px; --tile-y: {tileY}px; --tile-z: {tileZ}"
  onpointerdown={onPointerDown}
  onpointerup={onPointerUp}
>
  <button
    style="--tile-border-color: {config.icon_options.border_color}"
    bind:this={button}
    class="tile"
    class:tile--dragging={dragging}
    onclick={onClick}
    aria-roledescription="button"
    data-drop-zone="filledTile"
    data-row={tile.row}
    data-column={tile.column}
  >
    <TileIcon icon={config.icon} iconOptions={config.icon_options} />
    <TileLabelElm label={config.label} />
  </button>

  <span
    class="handle handle--top"
    use:resizeHandle={{
      direction: ResizeDirection.VERTICAL,
      distanceThreshold,
    }}
    onresize={handleResizeVerticalTop}
  ></span>

  <span
    class="handle handle--bottom"
    use:resizeHandle={{
      direction: ResizeDirection.VERTICAL,
      distanceThreshold,
    }}
    onresize={handleResizeVerticalBottom}
  ></span>

  <span
    class="handle handle--left"
    use:resizeHandle={{
      direction: ResizeDirection.HORIZONTAL,
      distanceThreshold,
    }}
    onresize={handleResizeHorizontalLeft}
  ></span>

  <span
    class="handle handle--corner-top-left"
    use:resizeHandle={{
      direction: ResizeDirection.DIAGONAL,
      distanceThreshold,
    }}
    onresize={considerResizeDiagonalLeftTop}
  ></span>

  <span
    class="handle handle--corner-bottom-left"
    use:resizeHandle={{
      direction: ResizeDirection.DIAGONAL,
      distanceThreshold,
    }}
    onresize={considerResizeDiagonalLeftBottom}
  ></span>

  <span
    class="handle handle--right"
    use:resizeHandle={{
      direction: ResizeDirection.HORIZONTAL,
      distanceThreshold,
    }}
    onresize={handleResizeHorizontalRight}
  ></span>

  <span
    class="handle handle--corner-top-right"
    use:resizeHandle={{
      direction: ResizeDirection.DIAGONAL,
      distanceThreshold,
    }}
    onresize={considerResizeDiagonalRightTop}
  ></span>
  <span
    class="handle handle--corner-bottom-right"
    use:resizeHandle={{
      direction: ResizeDirection.DIAGONAL,
      distanceThreshold,
    }}
    onresize={considerResizeDiagonalRightBottom}
  ></span>
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

  .tile {
    position: relative;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    width: var(--tile-width);
    height: var(--tile-height);

    color: #ccc;

    border: 2px solid var(--tile-border-color);
    cursor: pointer;

    background-color: #151318;

    user-select: none;
    overflow: hidden;
  }

  /* Disable pointer events for children to make dragging work properly */
  .tile > :global(*) {
    pointer-events: none;
  }

  .tile--dragging {
    opacity: 0.5;
  }

  .handle {
    position: absolute;
    background-color: gray;
  }

  .handle--top {
    top: 0;
    left: 8px;
    height: 8px;
    width: calc(100% - 24px);
    cursor: row-resize;
  }

  .handle--bottom {
    bottom: 0;
    left: 8px;
    height: 8px;
    width: calc(100% - 24px);
    cursor: row-resize;
  }

  .handle--left {
    top: 8px;
    left: 0;
    width: 8px;
    height: calc(100% - 24px);
    cursor: col-resize;
  }

  .handle--corner-top-left {
    top: 0;
    left: 0;
    height: 16px;
    width: 16px;
    cursor: nw-resize;
  }

  .handle--corner-bottom-left {
    bottom: 0;
    left: 0;
    height: 16px;
    width: 16px;
    cursor: sw-resize;
  }

  .handle--right {
    top: 8px;
    right: 0;
    width: 8px;
    height: calc(100% - 24px);
    cursor: col-resize;
  }

  .handle--corner-top-right {
    top: 0;
    right: 0;
    height: 16px;
    width: 16px;
    cursor: ne-resize;
  }

  .handle--corner-bottom-right {
    bottom: 0;
    right: 0;
    height: 16px;
    width: 16px;
    cursor: se-resize;
  }
</style>
