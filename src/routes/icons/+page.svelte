<script lang="ts">
  import type { IconPackId } from "$lib/api/types/icons";

  import { toast } from "svelte-sonner";
  import Aside from "$lib/components/Aside.svelte";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import SolarBoxBoldDuotone from "~icons/solar/box-bold-duotone";
  import SolarShopBoldDuotone from "~icons/solar/shop-bold-duotone";
  import IconPackCard from "$lib/components/icons/IconPackCard.svelte";
  import SkeletonList from "$lib/components/skeleton/SkeletonList.svelte";
  import { uninstallIconPack, createIconPacksQuery } from "$lib/api/icons";
  import { getErrorMessage, toastErrorMessage } from "$lib/api/utils/error";
  import ManualImportIconPack from "$lib/components/icons/ManualImportIconPack.svelte";

  const i18n = i18nContext.get();

  const iconPacksQuery = createIconPacksQuery();

  function handleUninstall(iconPackId: IconPackId) {
    const revokePromise = uninstallIconPack(iconPackId);

    toast.promise(revokePromise, {
      loading: i18n.f("icon_packs_uninstalling"),
      success: i18n.f("icon_packs_uninstalled"),
      error: toastErrorMessage(i18n.f("icon_packs_uninstall_error")),
    });
  }
</script>

<div class="layout">
  {#if iconPacksQuery.isLoading}
    <SkeletonList style="margin: 1rem" />
  {:else if iconPacksQuery.isError}
    <Aside severity="error" style="margin: 1rem">
      {i18n.f("icon_packs_installed_error", {
        values: { error: getErrorMessage(iconPacksQuery.error) },
      })}
    </Aside>
  {:else if iconPacksQuery.isSuccess}
    <div class="header">
      <div class="nav">
        <a class="tab tab--active" href="/icons">
          <SolarBoxBoldDuotone />

          {i18n.f("installed")}
        </a>
        <a class="tab" href="/icons/community">
          <SolarShopBoldDuotone />
          {i18n.f("community_icon_packs")}
        </a>
      </div>

      <div class="actions">
        <ManualImportIconPack />
      </div>
    </div>

    <div class="plugins-wrapper">
      <div class="plugins">
        {#each iconPacksQuery.data as pack, index (index)}
          {@const manifest = pack.manifest.icons}

          <IconPackCard
            version={manifest.version}
            name={manifest.name}
            description={manifest.description}
            authors={manifest.authors}
            onUninstall={() => handleUninstall(manifest.id)}
          />
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
    display: flex;
    gap: var(--tp-space-4);
    justify-content: space-between;
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

  .nav {
    display: flex;
    align-items: center;
    flex-shrink: 0;
    width: calc(24rem - var(--tp-space-4) * 2);
  }

  .tab {
    display: inline-flex;
    align-items: center;
    flex: auto;
    gap: var(--tp-space-2);
    justify-content: center;
    font-size: var(--tp-text-base);
    font-weight: var(--tp-font-weight-medium);
    text-decoration: none;
    color: var(--tp-text-primary);
    border-bottom: 2px solid transparent;
    height: var(--tp-btn-height-md);
    padding: 0 var(--tp-btn-padding-x-md);
    border-radius: var(--tp-radius-md);
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
  }

  .tab--active {
    background: var(--tp-bg-tertiary);
    border-bottom: 2px solid var(--tp-text-primary);
  }
</style>
