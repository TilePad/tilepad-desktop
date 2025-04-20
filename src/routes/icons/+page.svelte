<script lang="ts">
  import Aside from "$lib/components/Aside.svelte";
  import { createIconPacksQuery } from "$lib/api/icons";
  import { getErrorMessage } from "$lib/api/utils/error";
  import IconPackCard from "$lib/components/icons/IconPackCard.svelte";
  import ManualImportIconPack from "$lib/components/icons/ManualImportIconPack.svelte";

  const iconPacksQuery = createIconPacksQuery();
</script>

<div class="layout">
  {#if $iconPacksQuery.isLoading}
    <div class="skeleton-list">
      <div class="skeleton" style="width: 80%; height: 1rem"></div>
      <div class="skeleton" style="width: 70%; height: 1rem"></div>
      <div class="skeleton" style="width: 30%; height: 1rem"></div>
    </div>
  {:else if $iconPacksQuery.isError}
    <Aside severity="error" style="width: 100%">
      Failed to load icon packs: {getErrorMessage($iconPacksQuery.error)}
    </Aside>
  {:else if $iconPacksQuery.isSuccess && $iconPacksQuery.data.length > 0}
    <div class="header">
      <h2>Icon Packs</h2>
      <ManualImportIconPack />
    </div>

    <div class="plugins-wrapper">
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
    height: 100%;
    overflow: hidden;

    display: flex;
    flex-flow: column;
  }

  .header {
    display: flex;
    flex-flow: row;
    flex-shrink: 0;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem;
    border-bottom: 1px solid #333;
    background-color: #29262e;
  }

  .plugins-wrapper {
    flex: auto;
    overflow: auto;
  }

  .plugins {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    padding: 1rem;
  }
</style>
