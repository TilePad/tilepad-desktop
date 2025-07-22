<script lang="ts">
  import type { Snippet } from "svelte";

  type Props = {
    children?: Snippet;
  };

  const { children }: Props = $props();
</script>

<button
  style="--tile-size-adjustment: 1; --tile-width: 128px; --tile-height: 128px;"
  class="tile"
  aria-roledescription="button"
  data-drop-zone="filledTile"
>
  {@render children?.()}
</button>

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
  }
</style>
