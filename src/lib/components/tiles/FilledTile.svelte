<script lang="ts">
  import type { TileId, TileModel, TilePosition } from "$lib/api/types/tiles";

  import { watch, useDebounce } from "runed";
  import { type ResizeEventDetail } from "$lib/utils/resizable";
  import { createUpdateTilePositionMutation } from "$lib/api/tiles/tiles.mutations";
  import {
    type DisplayContext,
    CONTROLLER_DEVICE_ID,
  } from "$lib/api/types/plugin";

  import TileIcon from "./TileIcon.svelte";
  import EmptyTile from "./EmptyTile.svelte";
  import TileLabelElm from "./TileLabel.svelte";
  import TileContainer from "./TileContainer.svelte";
  import { getDraggingContext } from "./TileDraggingProvider.svelte";
  import TileDragHandles, { type ResizeSide } from "./TileDragHandles.svelte";

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

  /** Current persisted position of the tile */
  let tilePosition: TilePosition = $state(tile.position);

  /** Currently position of the tile (Affected by states such as dragging) */
  let currentPosition: TilePosition = $state(tile.position);

  watch(
    () => tile.position,
    (newPosition) => {
      tilePosition = newPosition;
      currentPosition = newPosition;
    },
  );

  let touchTimeout: number | undefined;
  let button: HTMLButtonElement | undefined = $state();
  let resizing = $state(false);

  const config = $derived(tile.config);
  const dragging = $derived.by(() => {
    const target = draggingState();
    if (target === null) return false;
    return target.data.type === "tile" && target.data.tileId === tile.id;
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
      onStartDragging(event, { type: "tile", tileId: tile.id }, button);
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
    tilePosition = { ...position };
    debounceUpdatePosition(position);
  }

  function handleResize(detail: ResizeEventDetail, side: ResizeSide) {
    resizing = true;

    let col = currentPosition.column;
    let row = currentPosition.row;

    let colSpan = currentPosition.column_span;
    let rowSpan = currentPosition.row_span;

    if (side === "top") {
      row =
        tilePosition.row + Math.min(detail.scaleY, tilePosition.row_span - 1);
      rowSpan = Math.max(tilePosition.row_span - detail.scaleY, 1);
    } else if (side === "bottom") {
      rowSpan = Math.max(tilePosition.row_span + detail.scaleY, 1);
    } else if (side === "left") {
      col =
        tilePosition.column +
        Math.min(detail.scaleX, tilePosition.column_span - 1);
      colSpan = Math.max(tilePosition.column_span - detail.scaleX, 1);
    } else if (side === "right") {
      colSpan = Math.max(tilePosition.column_span + detail.scaleX, 1);
    }

    if (!isAllowedWithin(col, colSpan, row, rowSpan, tile.id)) {
      return;
    }

    // Update position changes
    currentPosition = {
      column: col,
      column_span: colSpan,
      row: row,
      row_span: rowSpan,
    };

    if (detail.commit) {
      persistPosition(currentPosition);
      resizing = false;
    }
  }

  function handleResizeDiagonal(
    detail: ResizeEventDetail,
    sides: ResizeSide[],
  ) {
    const commit = detail.commit;
    detail.commit = false;
    resizing = true;

    for (const side of sides) {
      handleResize(detail, side);
    }

    if (commit) {
      persistPosition(currentPosition);
      resizing = false;
    }
  }
</script>

{#if resizing}
  <EmptyTile
    row={tilePosition.row}
    column={tilePosition.column}
    {tileSize}
    {gap}
  />
{/if}

<TileContainer position={currentPosition} {tileSize} {gap} {resizing}>
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
    <TileLabelElm {...config.label} />
  </button>

  <TileDragHandles
    {distanceThreshold}
    onResize={handleResize}
    onResizeDiagonal={handleResizeDiagonal}
  />
</TileContainer>

<style>
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
    transition: all 0.1s ease;
  }

  /* Disable pointer events for children to make dragging work properly */
  .tile > :global(*) {
    pointer-events: none;
  }

  .tile--dragging {
    opacity: 0.5;
  }
</style>
