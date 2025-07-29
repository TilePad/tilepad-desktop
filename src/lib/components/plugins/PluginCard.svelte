<script lang="ts">
  import type { PluginId, PluginTaskState } from "$lib/api/types/plugin";
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import SolarRefreshLinear from "~icons/solar/refresh-linear";
  import { createUpdatePlugin } from "$lib/api/plugins_registry";
  import { reloadPlugin } from "$lib/api/plugins/plugins.requests";
  import { createUninstallPlugin } from "$lib/api/plugins/plugins.mutations";

  import Button from "../input/Button.svelte";

  type Props = {
    id: PluginId;
    name: string;
    description: string | null;
    version: string;
    internal: boolean;
    authors: string[];
    state: PluginTaskState;
    latestVersion?: {
      version: string;
      remotePlugin: PluginRegistryEntry;
    };
    developerMode?: boolean;
  };

  const {
    id,
    name,
    description,
    version,
    internal,
    authors,
    state,
    latestVersion,
    developerMode,
  }: Props = $props();

  const uninstall = createUninstallPlugin();
  const update = createUpdatePlugin();

  function handleReload() {
    const reloadPromise = reloadPlugin(id);

    toast.promise(reloadPromise, {
      loading: $t("plugin_reloading"),
      success: $t("plugin_reloaded"),
      error: toastErrorMessage($t("plugin_reload_error")),
    });
  }

  function handleUninstall() {
    const uninstallPromise = $uninstall.mutateAsync({
      pluginId: id,
    });

    toast.promise(uninstallPromise, {
      loading: $t("plugin_uninstalling"),
      success: $t("plugin_uninstalled"),
      error: toastErrorMessage($t("plugin_uninstall_error")),
    });
  }

  async function handleUpdate() {
    if (!latestVersion) return;

    const updatePromise = $update.mutateAsync({
      repo: latestVersion.remotePlugin.repo,
      version: latestVersion.version,
      pluginId: id,
    });
    toast.promise(updatePromise, {
      loading: $t("plugin_updating"),
      success: $t("plugin_updated"),
      error: toastErrorMessage($t("plugin_update_error")),
    });
  }
</script>

<div class="card">
  <div class="head">
    <div class="head__text">
      <span class="version">
        {version}

        {#if latestVersion}
          <span class="version__new">
            New: {latestVersion.version}
          </span>
        {/if}
      </span>

      {#if developerMode && !internal}
        <span class="state">{state}</span>
      {/if}
    </div>
  </div>

  <h2 class="name">
    {name}
  </h2>

  {#if description}
    <p class="description">
      {description}
    </p>
  {/if}

  {#if authors.length > 0}
    <span class="authors">
      By {authors.join(", ")}
    </span>
  {/if}

  <div class="actions">
    {#if developerMode}
      <Button
        variant="secondary"
        title={$t("Reload")}
        size="small"
        onclick={handleReload}
      >
        <SolarRefreshLinear />
      </Button>
    {/if}

    {#if latestVersion}
      <Button
        variant="secondary"
        size="small"
        onclick={handleUpdate}
        loading={$update.isPending}
        disabled={$uninstall.isPending}
      >
        {$t("update")}
      </Button>
    {/if}

    {#if !internal}
      <Button
        variant="secondary"
        size="small"
        onclick={handleUninstall}
        loading={$uninstall.isPending}
        disabled={$update.isPending}
      >
        {$t("uninstall")}
      </Button>
    {/if}
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

  .head__text {
    display: flex;
    flex-flow: row;
    align-items: center;
    gap: var(--tp-space-2);
  }

  .version {
    color: var(--tp-text-tertiary);
    font-size: var(--tp-text-xs);
  }

  .version__new {
    display: inline-block;
    padding: 0.25rem 0.5rem;
    background-color: #6d5c92;

    margin-left: var(--tp-space-2);
    color: var(--tp-text-primary);
    border-radius: var(--tp-radius-lg);
  }

  .description {
    color: var(--tp-text-secondary);
    font-size: var(--tp-text-xs);
    max-width: 100%;
  }

  .name {
    font-size: var(--tp-text-lg);
    line-height: var(--tp-leading-tight);
  }

  .actions {
    display: flex;
    gap: var(--tp-space-2);
    flex-shrink: 0;
  }

  .state {
    vertical-align: middle;
    font-size: var(--tp-text-xs);
    color: var(--tp-text-tertiary);
  }

  .authors {
    font-size: var(--tp-text-xs);
    color: var(--tp-text-tertiary);
  }
</style>
