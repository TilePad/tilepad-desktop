<script lang="ts">
  import { getDraggingContext } from "./TileDraggingProvider.svelte";

  type Props = {
    row: number;
    column: number;
  };
  const { dropZoneTarget } = getDraggingContext();

  const { row, column }: Props = $props();

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
  class="tile"
  class:tile--dropping={dropping}
  data-drop-zone="emptyTile"
  data-row={row}
  data-column={column}
></div>

<style>
  .tile {
    position: relative;
    background-color: #242129;
    border: 2px solid #36313d;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    width: var(--tile-width);
    height: var(--tile-width);
    color: #ccc;
    user-select: none;

    user-select: none;
    overflow: hidden;

    font-size: 1.5rem;
    text-align: center;
  }

  .tile--dropping {
    background-color: #3a3542;
    border-color: #36313d;
  }
</style>
