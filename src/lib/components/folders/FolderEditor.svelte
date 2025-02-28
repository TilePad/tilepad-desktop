<script lang="ts">
  import TileGrid from "$lib/components/tiles/TileGrid.svelte";

  import Tile from "../tiles/Tile.svelte";
  import { getFolderContext } from "./FolderProvider.svelte";

  const { folder } = getFolderContext();
  const currentFolder = $derived.by(folder);
</script>

<div class="layout">
  <TileGrid
    rows={currentFolder.config.rows}
    columns={currentFolder.config.columns}
  >
    {#snippet tile(row, column)}
      <Tile {row} {column} />
    {/snippet}
  </TileGrid>
</div>

<style>
  .layout {
    display: flex;
    flex: auto;
    flex-flow: column;

    padding: 1rem;
    height: 100%;

    overflow: hidden;
  }
</style>
