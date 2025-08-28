<script lang="ts">
  import type { IconPackId } from "$lib/api/types/icons";
  import type { IconRegistryEntry } from "$lib/api/types/icons_registry";

  import { toast } from "svelte-sonner";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { createUninstallIconPack } from "$lib/api/icons";
  import { createUpdatePlugin } from "$lib/api/plugins_registry";

  import Button from "../input/Button.svelte";

  type Props = {
    id: IconPackId;
    version: string;
    name: string;
    description: string | null;
    authors: string[];
    latestVersion?: {
      version: string;
      remotePlugin: IconRegistryEntry;
    };
  };

  const { id, version, name, description, authors, latestVersion }: Props =
    $props();

  const i18n = i18nContext.get();

  const uninstall = createUninstallIconPack();
  const update = createUpdatePlugin();

  function handleUninstall() {
    const uninstallPromise = uninstall.mutateAsync({
      packId: id,
    });

    toast.promise(uninstallPromise, {
      loading: i18n.f("icon_packs_uninstalling"),
      success: i18n.f("icon_packs_uninstalled"),
      error: toastErrorMessage(i18n.f("icon_packs_uninstall_error")),
    });
  }

  async function handleUpdate() {
    if (!latestVersion) return;

    const updatePromise = update.mutateAsync({
      repo: latestVersion.remotePlugin.repo,
      version: latestVersion.version,
      pluginId: id,
    });
    toast.promise(updatePromise, {
      loading: i18n.f("plugin_updating"),
      success: i18n.f("plugin_updated"),
      error: toastErrorMessage(i18n.f("plugin_update_error")),
    });
  }
</script>

<div class="card">
  <div class="head">
    <span class="version">{version}</span>
  </div>

  <h2 class="name">
    {name}
  </h2>

  <p class="description">{description}</p>

  {#if authors.length > 0}
    <span class="authors">
      By {authors.join(", ")}
    </span>
  {/if}

  <div class="actions">
    {#if latestVersion}
      <Button
        variant="secondary"
        size="small"
        onclick={handleUpdate}
        loading={update.isPending}
        disabled={uninstall.isPending}
      >
        {i18n.f("update")}
      </Button>
    {/if}

    <Button
      variant="secondary"
      size="small"
      onclick={handleUninstall}
      loading={uninstall.isPending}
      disabled={update.isPending}
    >
      {i18n.f("uninstall")}
    </Button>
  </div>
</div>

<style>
  .card {
    display: flex;
    flex-flow: column;
    gap: var(--tp-space-2);
    align-items: flex-start;

    padding: var(--tp-space-3);
    border-radius: var(--tp-radius-md);
    background-color: var(--tp-bg-secondary);
    border: 1px solid var(--tp-border-secondary);
  }

  .head {
    display: flex;
    flex-flow: row;
    align-items: center;
    justify-content: space-between;
    gap: var(--tp-space-2);

    width: 100%;
  }

  .description {
    color: var(--tp-text-secondary);
    font-size: var(--tp-text-xs);
    max-width: 100%;
  }

  .version {
    color: var(--tp-text-tertiary);
    font-size: var(--tp-text-xs);
  }

  .name {
    font-size: var(--tp-text-lg);
    line-height: var(--tp-leading-tight);
  }

  .actions {
    display: flex;
    gap: var(--tp-space-3);
    flex-shrink: 0;
  }

  .authors {
    font-size: var(--tp-text-xs);
    color: var(--tp-text-tertiary);
  }
</style>
