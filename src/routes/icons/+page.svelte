<script lang="ts">
  import { createIconPacksQuery } from "$lib/api/icons";
  import IconPackCard from "$lib/components/icons/IconPackCard.svelte";
  import ManualImportIconPack from "$lib/components/icons/ManualImportIconPack.svelte";

  const iconPacksQuery = createIconPacksQuery();
</script>

<div class="layout">
  {#if $iconPacksQuery.isLoading}
    Loading icon packs...
  {:else if $iconPacksQuery.isError}
    Failed to load icon packs {$iconPacksQuery.error}
  {:else if $iconPacksQuery.isSuccess && $iconPacksQuery.data.length > 0}
    <div class="section">
      <div class="header">
        <h2>Icon Packs</h2>
        <ManualImportIconPack />
      </div>

      <div class="plugins">
        {#each $iconPacksQuery.data as pack}
          <IconPackCard {pack} />
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .layout {
    flex: auto;

    display: flex;
    flex-flow: column;
    gap: 0.5rem;

    padding: 1rem;
  }

  .header {
    display: flex;
    flex-flow: row;
    justify-content: space-between;
    align-items: center;
  }

  .section {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
  }

  .plugins {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
  }
</style>
