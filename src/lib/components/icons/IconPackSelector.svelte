<script lang="ts">
  import type { Icon, IconPackId } from "$lib/api/types/icons";

  import { t } from "svelte-i18n";
  import { createIconPacksQuery } from "$lib/api/icons";
  import { getIconAssetPath } from "$lib/api/utils/url";
  import { getErrorMessage } from "$lib/api/utils/error";

  import type { IconGridItem } from "./IconsGrid.svelte";

  import Aside from "../Aside.svelte";
  import TextInput from "../input/TextInput.svelte";
  import IconPackCategory from "./IconPackCategory.svelte";
  import SkeletonList from "../skeleton/SkeletonList.svelte";
  import { getServerContext } from "../ServerProvider.svelte";

  type IconPackData = {
    id: string;
    name: string;
    icons: IconGridItem[];
  };

  type Props = {
    onClickIcon: (packId: IconPackId, icon: Icon) => void;
  };

  const { onClickIcon }: Props = $props();

  const iconPacksQuery = createIconPacksQuery();

  const serverContext = getServerContext();
  const serverURL = $derived(serverContext.serverURL);

  let search = $state("");

  const iconPackData = $derived(
    (iconPacksQuery.data ?? []).map((item) => ({
      id: item.manifest.icons.id,
      name: item.manifest.icons.name,
      icons: item.icons.map((icon) => ({
        name: icon.name,
        path: icon.path,
        src: getIconAssetPath(serverURL, item.manifest.icons.id, icon.path),
      })),
    })),
  );

  const filteredPacks = $derived.by(() => {
    const query = search.trim();

    if (query.length < 1) {
      return iconPackData;
    }

    return filterIconPacks(iconPackData, query);
  });

  function filterIconPacks(packs: IconPackData[], query: string) {
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
  {#if iconPacksQuery.isLoading}
    <SkeletonList />
  {:else if iconPacksQuery.isError}
    <Aside severity="error" style="margin: 1rem">
      {$t("icon_packs_error", {
        values: { error: getErrorMessage(iconPacksQuery.error) },
      })}
    </Aside>
  {:else if iconPacksQuery.isSuccess}
    <TextInput
      fullWidth
      placeholder={$t("search_placeholder")}
      bind:value={search}
      style="margin-bottom: 8px"
    />

    <div class="categories">
      {#each filteredPacks as pack (pack.id)}
        <IconPackCategory
          name={pack.name}
          icons={pack.icons}
          onClickIcon={(icon) => onClickIcon(pack.id, icon)}
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
