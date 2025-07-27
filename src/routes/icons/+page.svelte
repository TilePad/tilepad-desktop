<script lang="ts">
  import { t } from "svelte-i18n";
  import Aside from "$lib/components/Aside.svelte";
  import { createIconPacksQuery } from "$lib/api/icons";
  import { getErrorMessage } from "$lib/api/utils/error";
  import SolarShopBoldDuotone from "~icons/solar/shop-bold-duotone";
  import IconPackCard from "$lib/components/icons/IconPackCard.svelte";
  import SkeletonList from "$lib/components/skeleton/SkeletonList.svelte";
  import ManualImportIconPack from "$lib/components/icons/ManualImportIconPack.svelte";
  import IconsRegistryDialog from "$lib/components/icons_registry/IconsRegistryDialog.svelte";

  const iconPacksQuery = createIconPacksQuery();
</script>

<div class="layout">
  {#if $iconPacksQuery.isLoading}
    <SkeletonList style="margin: 1rem" />
  {:else if $iconPacksQuery.isError}
    <Aside severity="error" style="margin: 1rem">
      {$t("icon_packs_installed_error", {
        values: { error: getErrorMessage($iconPacksQuery.error) },
      })}
    </Aside>
  {:else if $iconPacksQuery.isSuccess}
    <div class="header">
      <div class="actions">
        <IconsRegistryDialog
          buttonLabel={{
            text: $t("community_icon_packs"),
            icon: SolarShopBoldDuotone,
          }}
        />
        <ManualImportIconPack />
      </div>
    </div>

    <div class="plugins-wrapper">
      <div class="plugins">
        {#each $iconPacksQuery.data as pack, index (index)}
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
    padding: var(--tp-space-4);
  }

  .plugins-wrapper {
    flex: auto;
    overflow: auto;
  }

  .plugins {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(
      auto-fit,
      minmax(min(400px, max(200px, 100%)), 1fr)
    );
    gap: var(--tp-space-4);
    padding: var(--tp-space-4);
    padding-top: 0;
  }

  .actions {
    display: flex;
    gap: var(--tp-space-3);
  }
</style>
