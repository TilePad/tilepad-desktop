<script lang="ts">
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";
  import type { PluginManifest, PluginWithState } from "$lib/api/types/plugin";

  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import SolarRefreshLinear from "~icons/solar/refresh-linear";
  import { createUpdatePlugin } from "$lib/api/plugins_registry";
  import { reloadPlugin, createUninstallPlugin } from "$lib/api/plugins";

  import Button from "../input/Button.svelte";
  import { getSettingsContext } from "../SettingsProvider.svelte";

  type Props = {
    plugin: PluginWithState;
    latestManifest?: {
      manifest: PluginManifest;
      remotePlugin: PluginRegistryEntry;
    };
  };

  const { plugin, latestManifest }: Props = $props();
  const { manifest, state } = plugin;

  const settingsContext = getSettingsContext();
  const settings = $derived.by(settingsContext.settings);

  const uninstall = createUninstallPlugin();
  const update = createUpdatePlugin();

  function handleReload() {
    const reloadPromise = reloadPlugin(manifest.plugin.id);

    toast.promise(reloadPromise, {
      loading: $t("plugin_reloading"),
      success: $t("plugin_reloaded"),
      error: toastErrorMessage($t("plugin_reload_error")),
    });
  }

  function handleUninstall() {
    const uninstallPromise = $uninstall.mutateAsync({
      pluginId: manifest.plugin.id,
    });

    toast.promise(uninstallPromise, {
      loading: $t("plugin_uninstalling"),
      success: $t("plugin_uninstalled"),
      error: toastErrorMessage($t("plugin_uninstall_error")),
    });
  }

  async function handleUpdate() {
    if (!latestManifest) return;

    const updatePromise = $update.mutateAsync({
      repo: latestManifest.remotePlugin.repo,
      version: latestManifest.manifest.plugin.version,
      pluginId: latestManifest.manifest.plugin.id,
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
        {manifest.plugin.version}

        {#if latestManifest}
          <span class="version__new">
            New: {latestManifest.manifest.plugin.version}
          </span>
        {/if}
      </span>

      {#if settings.developer_mode && !manifest.plugin.internal}
        <span class="state">{state}</span>
      {/if}
    </div>
  </div>

  <h2 class="name">
    {manifest.plugin.name}
  </h2>

  <p class="description">
    {manifest.plugin.description}.
  </p>

  {#if manifest.plugin.authors.length > 0}
    <span class="authors">
      By {manifest.plugin.authors.join(", ")}
    </span>
  {/if}

  <div class="actions">
    {#if settings.developer_mode}
      <Button
        variant="secondary"
        title={$t("Reload")}
        size="small"
        onclick={handleReload}
      >
        <SolarRefreshLinear />
      </Button>
    {/if}

    {#if latestManifest}
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

    {#if !plugin.manifest.plugin.internal}
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
