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
  const { onStartDragging } = getDraggingContext();
  let touchTimeout: number | undefined;

  let button: HTMLButtonElement | undefined = $state();

  const config = $derived(tile.config);

  function onPointerDown(event: PointerEvent) {
    if (touchTimeout) {
      clearTimeout(touchTimeout);
      touchTimeout = undefined;
    }

    // Add event listeners to document for better drag handling
    document.addEventListener("pointerup", onPointerUp);

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
  bind:this={button}
  class="tile"
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

    border: 2px solid #715c8f;
    cursor: pointer;

    background-color: #151318;

    user-select: none;
    overflow: hidden;

    font-size: 1.5rem;
    text-align: center;
  }

  /* Disable pointer events for children to make dragging work properly */
  .tile > :global(*) {
    pointer-events: none;
  }
</style>
