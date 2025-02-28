<script lang="ts">
  import TileGrid from "$lib/components/tiles/TileGrid.svelte";

  import { getFolderContext } from "../profiles/FolderProvider.svelte";

  const { folder } = getFolderContext();
  const currentFolder = $derived.by(folder);
</script>

<div class="layout">
  <TileGrid
    rows={currentFolder.config.rows}
    columns={currentFolder.config.columns}
  >
    {#snippet tile(row, column)}
      <div class="tile"></div>
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

  .tile {
    background-color: #242129;
    border: 2px solid #36313d;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    color: white;
    font-weight: bold;
    text-align: center;
    cursor: pointer;
    width: 100%;
    height: 100%;
  }

  /* .tile--filled {
    background-color: #393441;
    border: 2px solid #534c5e;
  } */
</style>
