<script lang="ts">
  import type { PluginWithState } from "$lib/api/types/plugin";

  import { toast } from "svelte-sonner";
  import Aside from "$lib/components/Aside.svelte";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { compare as semverCompare } from "semver-ts";
  import { createPluginsQuery } from "$lib/api/plugins";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { createMutation } from "@tanstack/svelte-query";
  import { getPluginAssetPath } from "$lib/api/utils/url";
  import Button from "$lib/components/input/Button.svelte";
  import { serverContext } from "$lib/contexts/server.context";
  import SolarBoxBoldDuotone from "~icons/solar/box-bold-duotone";
  import SolarShopBoldDuotone from "~icons/solar/shop-bold-duotone";
  import PluginCard from "$lib/components/plugins/PluginCard.svelte";
  import { getLatestPluginVersions } from "$lib/api/plugins_registry";
  import SkeletonList from "$lib/components/skeleton/SkeletonList.svelte";
  import { getSettingsContext } from "$lib/components/SettingsProvider.svelte";
  import ManualImportPlugin from "$lib/components/plugins/ManualImportPlugin.svelte";

  const i18n = i18nContext.get();

  const settingsContext = getSettingsContext();
  const settings = $derived.by(settingsContext.settings);

  const currentServerContext = serverContext.get();

  const pluginsQuery = createPluginsQuery();

  const checkUpdatesMutation = createMutation(() => ({
    mutationFn: async ({ plugins }: { plugins: PluginWithState[] }) => {
      const latestVersions = await getLatestPluginVersions(plugins);
      const updates = [];

      for (const entry of latestVersions) {
        const localPlugin = plugins.find(
          (plugin) => plugin.manifest.plugin.id === entry.manifest.plugin.id,
        );

        if (
          localPlugin &&
          semverCompare(
            entry.manifest.plugin.version,
            localPlugin.manifest.plugin.version,
          ) === 1
        ) {
          updates.push(entry);
        }
      }

      toast.success(
        i18n.f("updates_found_count", { values: { count: updates.length } }),
      );

      return updates;
    },
  }));
</script>

<div class="layout">
  {#if pluginsQuery.isLoading}
    <SkeletonList style="margin: 1rem" />
  {:else if pluginsQuery.isError}
    <Aside severity="error" style="margin: 1rem">
      {i18n.f("plugins_error", {
        values: { error: getErrorMessage(pluginsQuery.error) },
      })}
    </Aside>
  {:else if pluginsQuery.isSuccess}
    <div class="header">
      <div class="nav">
        <a class="tab tab--active" href="/plugins">
          <SolarBoxBoldDuotone />

          {i18n.f("installed")}
        </a>
        <a class="tab" href="/plugins/community">
          <SolarShopBoldDuotone />
          {i18n.f("community_plugins")}
        </a>
      </div>

      <div class="actions">
        <Button
          variant="secondary"
          onclick={() => {
            checkUpdatesMutation.mutate({ plugins: pluginsQuery.data });
          }}
          loading={checkUpdatesMutation.isPending}
        >
          {i18n.f("check_for_updates")}
        </Button>

        <ManualImportPlugin />
      </div>
    </div>

    <div class="plugins-wrapper">
      <div class="plugins">
        {#each pluginsQuery.data as plugin (plugin.manifest.plugin.id)}
          {#if !plugin.manifest.plugin.internal || import.meta.env.DEV}
            {@const latestManifest = checkUpdatesMutation.data?.find(
              (entry) =>
                entry.manifest.plugin.id === plugin.manifest.plugin.id &&
                // Ignore if version already matches
                entry.manifest.plugin.version !==
                  plugin.manifest.plugin.version,
            )}
            {@const manifest = plugin.manifest.plugin}

            <PluginCard
              id={manifest.id}
              icon={manifest.icon
                ? getPluginAssetPath(
                    currentServerContext.serverURL,
                    manifest.id,
                    manifest.icon,
                  )
                : null}
              name={manifest.name}
              description={manifest.description}
              version={manifest.version}
              internal={manifest.internal ?? false}
              authors={manifest.authors}
              state={plugin.state}
              latestVersion={latestManifest
                ? {
                    version: latestManifest.manifest.plugin.version,
                    remotePlugin: latestManifest.remotePlugin,
                  }
                : undefined}
              developerMode={settings.developer_mode}
            />
          {/if}
        {:else}
          {i18n.f("plugins_none")}
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
