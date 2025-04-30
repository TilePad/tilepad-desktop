<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";

  import TileIcon from "./TileIcon.svelte";
  import TileLabelElm from "./TileLabelElm.svelte";
  import { getDraggingContext } from "./TileDraggingProvider.svelte";

  type Props = {
    tile: TileModel;

    onClick: VoidFunction;
  };

  const { tile, onClick }: Props = $props();
  const { draggingState, onStartDragging } = getDraggingContext();

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
</script>

<button
  style="--tile-border-color: {config.icon_options.border_color}"
  bind:this={button}
  class="tile"
  class:tile--dragging={dragging}
  onclick={onClick}
  aria-roledescription="button"
  onpointerdown={onPointerDown}
  onpointerup={onPointerUp}
  data-drop-zone="filledTile"
  data-row={tile.row}
  data-column={tile.column}
>
  <TileIcon icon={config.icon} iconOptions={config.icon_options} />
  <TileLabelElm label={config.label} />
</button>

<style>
  .tile {
    position: relative;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    width: var(--tile-width);
    height: var(--tile-width);
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
</style>
