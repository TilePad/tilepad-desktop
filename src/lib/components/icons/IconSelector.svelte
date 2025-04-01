<script lang="ts">
  import type { Icon, IconPack, IconPackId } from "$lib/api/types/icons";

  import { createIconPacksQuery } from "$lib/api/icons";
  import { getErrorMessage } from "$lib/api/utils/error";

  import TextInput from "../input/TextInput.svelte";
  import IconPackCategory from "./IconPackCategory.svelte";
  import PopoverButton from "../popover/PopoverButton.svelte";

  type Props = { onClickIcon: (packId: IconPackId, icon: Icon) => void };

  const { onClickIcon }: Props = $props();

  const iconPacksQuery = createIconPacksQuery();

  let search = $state("");
  const filteredPacks = $derived.by(() => {
    const data = $iconPacksQuery.data ?? [];
    const query = search.trim();

    if (query.length < 1) {
      return data;
    }

    return filterIconPacks(data, query);
  });

  function filterIconPacks(packs: IconPack[], query: string) {
    query = query.toLowerCase();

    return packs
      .map((pack) => {
        return {
          ...pack,
          icons: pack.icons.filter((icon) => {
            const name = icon.name.toLowerCase();
            return name === query || name.includes(query);
          }),
        };
      })
      .filter((pack) => pack.icons.length > 0);
  }
</script>

<PopoverButton>
  {#snippet button({ props })}
    <button class="btn" {...props} type="button"> + </button>
  {/snippet}

  {#snippet content()}
    <div class="content">
      {#if $iconPacksQuery.isLoading}
        <p>Loading...</p>
      {:else if $iconPacksQuery.isError}
        <p>
          Failed to load icon packs: {getErrorMessage($iconPacksQuery.error)}
        </p>
      {:else if $iconPacksQuery.isSuccess}
        <TextInput placeholder="Search" bind:value={search} />

        <div class="categories">
          {#each filteredPacks as pack}
            <IconPackCategory
              onClickIcon={(icon) => onClickIcon(pack.manifest.icons.id, icon)}
              {pack}
            />
          {/each}
        </div>
      {/if}
    </div>
  {/snippet}
</PopoverButton>

<style>
  .content {
    width: 30rem;
  }

  .categories {
    max-height: 30rem;
    overflow-y: auto;
    overflow-x: hidden;
  }

  .btn {
    position: absolute;
    right: 0;
    top: 0;
    z-index: 1;
    cursor: pointer;

    width: 1.5rem;
    height: 1.5rem;

    padding: 0.35rem;
    background-color: #544d5e;
    border: none;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;
  }
</style>
