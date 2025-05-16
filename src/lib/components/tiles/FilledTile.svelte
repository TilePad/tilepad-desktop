<script lang="ts">
  import type { TileId, TileModel, TilePosition } from "$lib/api/types/tiles";

  import { useDebounce } from "runed";
  import { createUpdateTilePositionMutation } from "$lib/api/tiles";
  import {
    type DisplayContext,
    CONTROLLER_DEVICE_ID,
  } from "$lib/api/types/plugin";
  import {
    resizeHandle,
    ResizeDirection,
    type ResizeEventDetail,
  } from "$lib/utils/resizable";

  import TileIcon from "./TileIcon.svelte";
  import EmptyTile from "./EmptyTile.svelte";
  import TileLabelElm from "./TileLabel.svelte";
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

  const displayCtx: DisplayContext = $derived({
    device_id: CONTROLLER_DEVICE_ID,
    tile_id: tile.id,
    action_id: tile.action_id,
    plugin_id: tile.plugin_id,
  });

  const updateTilePosition = createUpdateTilePositionMutation();

  let lastPosition: TilePosition = $state(tile.position);
  let position: TilePosition = $state(tile.position);

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
  let resizing = $state(false);

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
    resizing = true;
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
      resizing = false;
    }
  }

  function handleResizeVerticalBottom(event: CustomEvent<ResizeEventDetail>) {
    resizing = true;
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
      resizing = false;
    }
  }

  function handleResizeHorizontalLeft(event: CustomEvent<ResizeEventDetail>) {
    resizing = true;
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
      resizing = false;
    }
  }

  function handleResizeHorizontalRight(event: CustomEvent<ResizeEventDetail>) {
    resizing = true;
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
      resizing = false;
    }
  }

  function considerResizeDiagonalLeftTop(
    event: CustomEvent<ResizeEventDetail>,
  ) {
    let commit = event.detail.commit;
    event.detail.commit = false;
    resizing = true;

    handleResizeHorizontalLeft(event);
    handleResizeVerticalTop(event);

    if (commit) {
      persistPosition(position);
      resizing = false;
    }
  }

  function considerResizeDiagonalLeftBottom(
    event: CustomEvent<ResizeEventDetail>,
  ) {
    let commit = event.detail.commit;
    event.detail.commit = false;
    resizing = true;

    handleResizeHorizontalLeft(event);
    handleResizeVerticalBottom(event);

    if (commit) {
      persistPosition(position);
      resizing = false;
    }
  }

  function considerResizeDiagonalRightBottom(
    event: CustomEvent<ResizeEventDetail>,
  ) {
    let commit = event.detail.commit;
    event.detail.commit = false;
    resizing = true;

    handleResizeHorizontalRight(event);
    handleResizeVerticalBottom(event);

    if (commit) {
      persistPosition(position);
      resizing = false;
    }
  }

  function considerResizeDiagonalRightTop(
    event: CustomEvent<ResizeEventDetail>,
  ) {
    let commit = event.detail.commit;
    event.detail.commit = false;
    resizing = true;

    handleResizeHorizontalRight(event);
    handleResizeVerticalTop(event);

    if (commit) {
      persistPosition(position);
      resizing = false;
    }
  }
</script>

{#if resizing}
  <EmptyTile
    row={lastPosition.row}
    column={lastPosition.column}
    width={tileSize}
    {gap}
  />
{/if}

<div
  class:tile-container--resizing={resizing}
  class="tile-container"
  style="--tile-size-adjustment: {sizeAdjust}; --tile-width: {tileWidth}px; --tile-height: {tileHeight}px; --tile-x: {tileX}px; --tile-y: {tileY}px; --tile-z: {tileZ}"
>
  <button
    onpointerdown={onPointerDown}
    onpointerup={onPointerUp}
    onclick={onClick}
    style="--tile-border-color: {config.icon_options.border_color}"
    bind:this={button}
    class="tile"
    class:tile--dragging={dragging}
    aria-roledescription="button"
    data-drop-zone="filledTile"
    data-row={tile.position.row}
    data-column={tile.position.column}
  >
    <TileIcon
      ctx={displayCtx}
      icon={config.icon}
      iconOptions={config.icon_options}
    />
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

  .tile-container--resizing {
    transition: all 0.1s ease;
  }

  .tile-container--resizing .tile {
    transition: all 0.1s ease;
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
    --handle-width: 1px;
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
