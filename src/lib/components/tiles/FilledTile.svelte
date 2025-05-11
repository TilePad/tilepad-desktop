<script lang="ts">
  import type { TileId, TileModel, TilePosition } from "$lib/api/types/tiles";

  import { useDebounce } from "runed";
  import { createUpdateTilePositionMutation } from "$lib/api/tiles";
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

    isAllowedWithin: (
      col: number,
      colSpan: number,
      row: number,
      rowSpan: number,
      exclude: TileId,
    ) => boolean;
  };

  const { tile, tileSize, gap, onClick, isAllowedWithin }: Props = $props();
  const { draggingState, onStartDragging } = getDraggingContext();

  const updateTilePosition = createUpdateTilePositionMutation();

  let lastPosition: TilePosition = $state(tile.position);
  let position: TilePosition = $state(tile.position);

  const { tileX, tileY, tileZ, tileWidth, tileHeight, sizeAdjust } =
    $derived.by(() => {
      const tileWidth =
        tileSize * position.column_span + gap * (position.column_span - 1);
      const tileHeight =
        tileSize * position.row_span + gap * (position.row_span - 1);

      // When measuring the desired width/height the ratio must first remove the gap size
      const desiredWidth =
        DESIRED_TILE_WIDTH - gap * (position.column_span - 1);
      const desiredHeight = DESIRED_TILE_WIDTH - gap * (position.row_span - 1);

      const ratioX = (tileWidth - desiredWidth) / desiredWidth;
      const ratioY = (tileHeight - desiredHeight) / desiredHeight;

      const sizeAdjustX = 1 + ratioX;
      const sizeAdjustY = 1 + ratioY;
      const sizeAdjust = Math.min(sizeAdjustX, sizeAdjustY);

      if (tile.config.label.label === "Pause recording") {
        console.log(tile, sizeAdjust, sizeAdjustX, sizeAdjustY);
      }

      const tileX = tileSize * position.column + gap * position.column;
      const tileY = tileSize * position.row + gap * position.row;
      const tileZ =
        (position.row + position.row_span) *
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

  let touchTimeout: number | undefined;
  let button: HTMLButtonElement | undefined = $state();

  const config = $derived(tile.config);
  const dragging = $derived.by(() => {
    const target = draggingState();
    if (target === null) return false;
    return target.data.type === "tile" && target.data.id === tile.id;
  });
  const distanceThreshold = $derived(tileSize + gap);

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

  const debounceUpdatePosition = useDebounce((position: TilePosition) => {
    $updateTilePosition.mutate({
      tileId: tile.id,
      position,
    });
  }, 100);

  function persistPosition(position: TilePosition) {
    lastPosition = { ...position };
    debounceUpdatePosition(position);
  }

  function handleResizeVerticalTop(event: CustomEvent<ResizeEventDetail>) {
    const newStart =
      lastPosition.row +
      Math.min(event.detail.scaleY, lastPosition.row_span - 1);
    const newSpan = Math.max(lastPosition.row_span - event.detail.scaleY, 1);

    // Prevent overlapping
    if (
      !isAllowedWithin(
        position.column,
        position.column_span,
        newStart,
        newSpan,
        tile.id,
      )
    ) {
      return;
    }

    position = { ...position, row: newStart, row_span: newSpan };

    if (event.detail.commit) {
      persistPosition(position);
    }
  }

  function handleResizeVerticalBottom(event: CustomEvent<ResizeEventDetail>) {
    const newSpan = Math.max(lastPosition.row_span + event.detail.scaleY, 1);

    // Prevent overlapping
    if (
      !isAllowedWithin(
        position.column,
        position.column_span,
        position.row,
        newSpan,
        tile.id,
      )
    ) {
      return;
    }

    position = { ...position, row_span: newSpan };

    if (event.detail.commit) {
      persistPosition(position);
    }
  }

  function handleResizeHorizontalLeft(event: CustomEvent<ResizeEventDetail>) {
    const newStart =
      lastPosition.column +
      Math.min(event.detail.scaleX, lastPosition.column_span - 1);
    const newSpan = Math.max(lastPosition.column_span - event.detail.scaleX, 1);

    // Prevent overlapping
    if (
      !isAllowedWithin(
        newStart,
        newSpan,
        position.row,
        position.row_span,
        tile.id,
      )
    ) {
      return;
    }

    position = { ...position, column: newStart, column_span: newSpan };

    if (event.detail.commit) {
      persistPosition(position);
    }
  }

  function handleResizeHorizontalRight(event: CustomEvent<ResizeEventDetail>) {
    const newSpan = Math.max(lastPosition.column_span + event.detail.scaleX, 1);

    // Prevent overlapping
    if (
      !isAllowedWithin(
        position.column,
        newSpan,
        position.row,
        position.row_span,
        tile.id,
      )
    ) {
      return;
    }

    position = { ...position, column_span: newSpan };

    if (event.detail.commit) {
      persistPosition(position);
    }
  }

  function considerResizeDiagonalLeftTop(
    event: CustomEvent<ResizeEventDetail>,
  ) {
    let commit = event.detail.commit;
    event.detail.commit = false;

    handleResizeHorizontalLeft(event);
    handleResizeVerticalTop(event);

    if (commit) {
      persistPosition(position);
    }
  }

  function considerResizeDiagonalLeftBottom(
    event: CustomEvent<ResizeEventDetail>,
  ) {
    let commit = event.detail.commit;
    event.detail.commit = false;

    handleResizeHorizontalLeft(event);
    handleResizeVerticalBottom(event);

    if (commit) {
      persistPosition(position);
    }
  }

  function considerResizeDiagonalRightBottom(
    event: CustomEvent<ResizeEventDetail>,
  ) {
    let commit = event.detail.commit;
    event.detail.commit = false;

    handleResizeHorizontalRight(event);
    handleResizeVerticalBottom(event);

    if (commit) {
      persistPosition(position);
    }
  }

  function considerResizeDiagonalRightTop(
    event: CustomEvent<ResizeEventDetail>,
  ) {
    let commit = event.detail.commit;
    event.detail.commit = false;

    handleResizeHorizontalRight(event);
    handleResizeVerticalTop(event);

    if (commit) {
      persistPosition(position);
    }
  }
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
    data-row={tile.position.row}
    data-column={tile.position.column}
  >
    <TileIcon icon={config.icon} iconOptions={config.icon_options} />
    <TileLabelElm label={config.label} />
  </button>

  <span
    class="handle handle--vertical handle--top"
    use:resizeHandle={{
      direction: ResizeDirection.VERTICAL,
      distanceThreshold,
    }}
    onresize={handleResizeVerticalTop}
  ></span>

  <span
    class="handle handle--vertical handle--bottom"
    use:resizeHandle={{
      direction: ResizeDirection.VERTICAL,
      distanceThreshold,
    }}
    onresize={handleResizeVerticalBottom}
  ></span>

  <span
    class="handle handle--horizontal handle--left"
    use:resizeHandle={{
      direction: ResizeDirection.HORIZONTAL,
      distanceThreshold,
    }}
    onresize={handleResizeHorizontalLeft}
  ></span>

  <span
    class="handle handle--corner handle--corner-top-left"
    use:resizeHandle={{
      direction: ResizeDirection.DIAGONAL,
      distanceThreshold,
    }}
    onresize={considerResizeDiagonalLeftTop}
  ></span>

  <span
    class="handle handle--corner handle--corner-bottom-left"
    use:resizeHandle={{
      direction: ResizeDirection.DIAGONAL,
      distanceThreshold,
    }}
    onresize={considerResizeDiagonalLeftBottom}
  ></span>

  <span
    class="handle handle--horizontal handle--right"
    use:resizeHandle={{
      direction: ResizeDirection.HORIZONTAL,
      distanceThreshold,
    }}
    onresize={handleResizeHorizontalRight}
  ></span>

  <span
    class="handle handle--corner handle--corner-top-right"
    use:resizeHandle={{
      direction: ResizeDirection.DIAGONAL,
      distanceThreshold,
    }}
    onresize={considerResizeDiagonalRightTop}
  ></span>
  <span
    class="handle handle--corner handle--corner-bottom-right"
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
    --handle-width: 4px;
    --handle-offset: 1px;
    --handle-corner-size: calc(var(--handle-width) * 2);

    position: absolute;
  }

  .handle:hover {
    background-color: #fff;
  }

  .handle--vertical {
    left: var(--handle-corner-size);
    height: var(--handle-width);
    width: calc(100% - (var(--handle-corner-size) * 2) - var(--handle-offset));
    cursor: row-resize;
  }

  .handle--top {
    top: var(--handle-offset);
  }

  .handle--bottom {
    bottom: var(--handle-offset);
  }

  .handle--horizontal {
    height: calc(100% - (var(--handle-corner-size) * 2) - var(--handle-offset));
    top: var(--handle-corner-size);
    width: var(--handle-width);
    cursor: col-resize;
  }

  .handle--left {
    left: var(--handle-offset);
  }

  .handle--right {
    right: var(--handle-offset);
  }

  .handle--corner {
    height: var(--handle-corner-size);
    width: var(--handle-corner-size);
  }

  .handle--corner-top-left {
    top: var(--handle-offset);
    left: var(--handle-offset);
    cursor: nw-resize;
  }

  .handle--corner-bottom-left {
    bottom: var(--handle-offset);
    left: var(--handle-offset);
    cursor: sw-resize;
  }

  .handle--corner-top-right {
    top: var(--handle-offset);
    right: var(--handle-offset);
    cursor: ne-resize;
  }

  .handle--corner-bottom-right {
    bottom: var(--handle-offset);
    right: var(--handle-offset);
    cursor: se-resize;
  }
</style>
