<script lang="ts">
  import type { PluginWithState } from "$lib/api/types/plugin";

  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import Aside from "$lib/components/Aside.svelte";
  import { compare as semverCompare } from "semver-ts";
  import { createPluginsQuery } from "$lib/api/plugins";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { createMutation } from "@tanstack/svelte-query";
  import Button from "$lib/components/input/Button.svelte";
  import SolarShopBoldDuotone from "~icons/solar/shop-bold-duotone";
  import PluginCard from "$lib/components/plugins/PluginCard.svelte";
  import SkeletonList from "$lib/components/skeleton/SkeletonList.svelte";
  import ManualImportPlugin from "$lib/components/plugins/ManualImportPlugin.svelte";
  import PluginsRegistryDialog from "$lib/components/plugins_registry/PluginsRegistryDialog.svelte";
  import {
    fetchPluginManifest,
    fetchPluginRegistry,
  } from "$lib/api/plugins_registry";

  const pluginsQuery = createPluginsQuery();

  const checkUpdatesMutation = createMutation({
    mutationFn: async ({ plugins }: { plugins: PluginWithState[] }) => {
      // Load remote available plugins
      const remotePlugins = await fetchPluginRegistry();

      // Find the remote plugins that are currently installed
      const installedPlugins = remotePlugins.filter((remotePlugin) => {
        return plugins.find(
          (plugin) => remotePlugin.id === plugin.manifest.plugin.id,
        );
      });

      //
      const updates = [];

      // Process in batches of 5
      for (let i = 0; i < installedPlugins.length; i += 5) {
        // Fetch all the manifests for the installed plugins
        const remotePluginSet = installedPlugins.slice(i, i + 5);
        const remotePluginManifests = await Promise.all(
          remotePluginSet.map(async (remotePlugin) => {
            const manifest = await fetchPluginManifest(remotePlugin.repo);
            return { manifest, remotePlugin };
          }),
        );

        for (const entry of remotePluginManifests) {
          const localPlugin = plugins.find(
            (plugin) => plugin.manifest.plugin.id === entry.manifest.plugin.id,
          );

          if (!localPlugin) continue;

          if (
            semverCompare(
              entry.manifest.plugin.version,
              localPlugin.manifest.plugin.version,
            ) === 1
          ) {
            updates.push(entry);
          }
        }
      }

      toast.success(
        $t("updates_found_count", { values: { count: updates.length } }),
      );

      return updates;
    },
  });
</script>

<div class="layout">
  {#if $pluginsQuery.isLoading}
    <SkeletonList style="margin: 1rem" />
  {:else if $pluginsQuery.isError}
    <Aside severity="error" style="margin: 1rem">
      {$t("plugins_error", {
        values: { error: getErrorMessage($pluginsQuery.error) },
      })}
    </Aside>
  {:else if $pluginsQuery.isSuccess}
    <div class="header">
      <h2>{$t("plugins")}</h2>
      <div class="actions">
        <Button
          onclick={() => {
            $checkUpdatesMutation.mutate({ plugins: $pluginsQuery.data });
          }}
          loading={$checkUpdatesMutation.isPending}
        >
          Check for updates
        </Button>
        <PluginsRegistryDialog
          buttonLabel={{
            text: $t("community_plugins"),
            icon: SolarShopBoldDuotone,
          }}
        />
        <ManualImportPlugin />
      </div>
    </div>

    <div class="plugins-wrapper">
      <div class="plugins">
        {#each $pluginsQuery.data as plugin (plugin.manifest.plugin.id)}
          {#if !plugin.manifest.plugin.internal || import.meta.env.DEV}
            <PluginCard
              {plugin}
              latestManifest={$checkUpdatesMutation.data?.find(
                (entry) =>
                  entry.manifest.plugin.id === plugin.manifest.plugin.id &&
                  // Ignore if version already matches
                  entry.manifest.plugin.version !==
                    plugin.manifest.plugin.version,
              )}
            />
          {/if}
        {:else}
          {$t("plugins_none")}
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
    display: flex;
    flex-flow: row;
    flex-shrink: 0;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem;
    border-bottom: 1px solid #333;
    background-color: #29262e;
  }

  .plugins-wrapper {
    flex: auto;
    overflow: auto;
  }

  .plugins {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    padding: 1rem;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }
</style>
