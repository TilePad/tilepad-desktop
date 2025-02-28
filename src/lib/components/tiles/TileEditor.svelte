<script lang="ts">
  import type { TilePosition } from "$lib/types";

  type Props = {
    tile: TilePosition | null;
  };

  const { tile }: Props = $props();
</script>

<div class="editor">
  <h1>Configure Tile ({tile?.row}, {tile?.column})</h1>
</div>

<style>
  .editor {
    position: relative;
    height: 250px;
    background-color: #28252c;
    flex: auto;
    width: 100%;
    padding: 2rem;
    border-top: 2px solid #302d36;
    flex-grow: 0;
    flex-shrink: 0;
  }
</style>
