<script lang="ts">
  import type { PluginId, PluginTaskState } from "$lib/api/types/plugin";
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  import { toast } from "svelte-sonner";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
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

  const i18n = i18nContext.get();

  const uninstall = createUninstallPlugin();
  const update = createUpdatePlugin();

  function handleReload() {
    const reloadPromise = reloadPlugin(id);

    toast.promise(reloadPromise, {
      loading: i18n.f("plugin_reloading"),
      success: i18n.f("plugin_reloaded"),
      error: toastErrorMessage(i18n.f("plugin_reload_error")),
    });
  }

  function handleUninstall() {
    const uninstallPromise = uninstall.mutateAsync({
      pluginId: id,
    });

    toast.promise(uninstallPromise, {
      loading: i18n.f("plugin_uninstalling"),
      success: i18n.f("plugin_uninstalled"),
      error: toastErrorMessage(i18n.f("plugin_uninstall_error")),
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
        title={i18n.f("Reload")}
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
        loading={update.isPending}
        disabled={uninstall.isPending}
      >
        {i18n.f("update")}
      </Button>
    {/if}

    {#if !internal}
      <Button
        variant="secondary"
        size="small"
        onclick={handleUninstall}
        loading={uninstall.isPending}
        disabled={update.isPending}
      >
        {i18n.f("uninstall")}
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
