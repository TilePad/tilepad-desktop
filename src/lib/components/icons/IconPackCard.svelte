<script lang="ts">
  import type { IconPack } from "$lib/api/types/icons";

  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { uninstallIconPack } from "$lib/api/icons";
  import { toastErrorMessage } from "$lib/api/utils/error";

  import Button from "../input/Button.svelte";

  type Props = {
    pack: IconPack;
  };

  const { pack }: Props = $props();
  const manifest = $derived(pack.manifest);

  function handleUninstall() {
    const revokePromise = uninstallIconPack(manifest.icons.id);

    toast.promise(revokePromise, {
      loading: $t("icon_packs_uninstalling"),
      success: $t("icon_packs_uninstalled"),
      error: toastErrorMessage($t("icon_packs_uninstall_error")),
    });
  }
</script>

<div class="card">
  <div class="head">
    <span class="version">{manifest.icons.version}</span>
  </div>

  <h2 class="name">
    {manifest.icons.name}
  </h2>

  <p class="description">{manifest.icons.description}</p>

  {#if manifest.icons.authors.length > 0}
    <span class="authors">
      By {manifest.icons.authors.join(", ")}
    </span>
  {/if}

  <div class="actions">
    <Button variant="secondary" size="small" onclick={handleUninstall}>
      {$t("uninstall")}
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
