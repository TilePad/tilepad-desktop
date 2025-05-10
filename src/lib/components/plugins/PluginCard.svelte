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

<div class="plugin">
  <div class="top">
    <span class="plugin__version">
      {manifest.plugin.version}

      {#if latestManifest}
        <span class="new-version">
          New: {latestManifest.manifest.plugin.version}
        </span>
      {/if}
    </span>

    <div class="plugin__actions"></div>
  </div>

  <h2 class="plugin__name">
    {manifest.plugin.name}
  </h2>

  {#if manifest.plugin.authors.length > 0}
    <span class="authors">
      By {manifest.plugin.authors.join(", ")}
    </span>
  {/if}

  <p class="plugin__description">
    {manifest.plugin.description}.
  </p>

  {#if settings.developer_mode && !manifest.plugin.internal}
    <span class="state">{state}</span>
  {/if}

  <div class="plugin__actions">
    {#if settings.developer_mode}
      <Button title={$t("Reload")} size="small" onclick={handleReload}>
        <SolarRefreshLinear />
      </Button>
    {/if}

    {#if latestManifest}
      <Button
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
  .top {
    display: flex;
    flex-flow: row;
    justify-content: space-between;
    align-items: center;
    width: 100%;
  }

  .plugin {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
    align-items: flex-start;

    padding: 1rem;
    border-radius: 0.5rem;
    background-color: #2f2c36;
  }

  .plugin__description {
    color: #ccc;
    font-size: 0.8rem;
    max-width: 100%;
  }

  .plugin__version {
    color: #ccc;
    font-size: 0.8rem;
  }

  .new-version {
    display: inline-block;
    padding: 0.25rem 0.5rem;
    background-color: #6d5c92;
    margin-left: 0.5rem;
    color: #fff;
    border-radius: 0.5rem;
  }

  .plugin__name {
    font-size: 1.2rem;
    margin-bottom: 0;
    line-height: 1;
  }

  .plugin__actions {
    display: flex;
    gap: 0.75rem;
  }

  .state {
    display: inline-flex;
    gap: 0.5rem;
    font-size: 0.8rem;
    vertical-align: middle;
    color: #999;
  }

  .authors {
    font-size: 0.9rem;
    color: #999;
  }
</style>
