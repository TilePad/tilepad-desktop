<script lang="ts">
  import type { Icon, IconPackId } from "$lib/api/types/icons";

  import { createIconPacksQuery } from "$lib/api/icons";
  import { getErrorMessage } from "$lib/api/utils/error";

  import IconPackCategory from "./IconPackCategory.svelte";
  import PopoverButton from "../popover/PopoverButton.svelte";

  type Props = { onClickIcon: (packId: IconPackId, icon: Icon) => void };

  const { onClickIcon }: Props = $props();

  const iconPacksQuery = createIconPacksQuery();
</script>

<PopoverButton>
  Icon

  {#snippet content()}
    <div>
      {#if $iconPacksQuery.isLoading}
        <p>Loading...</p>
      {:else if $iconPacksQuery.isError}
        <p>
          Failed to load icon packs: {getErrorMessage($iconPacksQuery.error)}
        </p>
      {:else if $iconPacksQuery.isSuccess}
        <div>
          {#each $iconPacksQuery.data as pack}
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
