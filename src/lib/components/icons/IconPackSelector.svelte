<script lang="ts">
  import type { Icon, IconPack, IconPackId } from "$lib/api/types/icons";

  import { t } from "svelte-i18n";
  import { createIconPacksQuery } from "$lib/api/icons";
  import { getErrorMessage } from "$lib/api/utils/error";

  import Aside from "../Aside.svelte";
  import TextInput from "../input/TextInput.svelte";
  import IconPackCategory from "./IconPackCategory.svelte";
  import SkeletonList from "../skeleton/SkeletonList.svelte";

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

<div class="content">
  {#if $iconPacksQuery.isLoading}
    <SkeletonList />
  {:else if $iconPacksQuery.isError}
    <Aside severity="error" style="margin: 1rem">
      {$t("icon_packs_error", {
        values: { error: getErrorMessage($iconPacksQuery.error) },
      })}
    </Aside>
  {:else if $iconPacksQuery.isSuccess}
    <TextInput
      fullWidth
      placeholder={$t("search_placeholder")}
      bind:value={search}
      style="margin-bottom: 8px"
    />

    <div class="categories">
      {#each filteredPacks as pack (pack.manifest.icons.id)}
        <IconPackCategory
          onClickIcon={(icon) => onClickIcon(pack.manifest.icons.id, icon)}
          {pack}
        />
      {/each}
    </div>
  {/if}
</div>

<style>
  .content {
    width: 30rem;
  }

  .categories {
    max-height: 70vh;
    overflow-y: auto;
    overflow-x: hidden;
  }
</style>
