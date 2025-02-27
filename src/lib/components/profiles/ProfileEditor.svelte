<script lang="ts">
  import TileGrid from "$lib/components/tiles/TileGrid.svelte";

  let rows = $state(4);
  let columns = $state(6);
</script>

<div class="layout">
  <div class="grid-container">
    <TileGrid {rows} {columns}>
      {#snippet tile(row, column)}
        <div class="tile"></div>
      {/snippet}
    </TileGrid>
  </div>
</div>

<style>
  .layout {
    display: flex;
    flex-flow: column;
    height: 100%;
    overflow: hidden;
  }

  .grid-container {
    display: flex;
    flex: auto;
    overflow: hidden;
    padding: 1rem;
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

  .tile--filled {
    background-color: #393441;
    border: 2px solid #534c5e;
  }
</style>
