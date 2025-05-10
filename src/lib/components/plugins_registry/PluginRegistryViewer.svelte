<script lang="ts">
  import type { PluginManifest } from "$lib/api/types/plugin";
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { compare as semverCompare } from "semver-ts";
  import { replaceMarkdownRelativeUrls } from "$lib/utils/markdown";
  import { uninstallPlugin, installPluginBuffer } from "$lib/api/plugins";
  import { getErrorMessage, toastErrorMessage } from "$lib/api/utils/error";
  import {
    getPluginBundle,
    createPluginReadmeQuery,
    createPluginManifestQuery,
    createInstallPluginFromRegistry,
  } from "$lib/api/plugins_registry";

  import Aside from "../Aside.svelte";
  import Button from "../input/Button.svelte";
  import Markdown from "../markdown/Markdown.svelte";
  import SkeletonList from "../skeleton/SkeletonList.svelte";

  type Props = {
    item: PluginRegistryEntry;
    installed?: PluginManifest;
  };

  const { item, installed }: Props = $props();

  const manifestQuery = createPluginManifestQuery(() => item.repo);
  const readmeQuery = createPluginReadmeQuery(() => item.repo);

  const install = createInstallPluginFromRegistry();

  async function onInstall() {
    const manifest = $manifestQuery.data;
    if (!manifest) return;

    const installPromise = $install.mutateAsync(
      {
        repo: item.repo,
        version: manifest.plugin.version,
      },
      {},
    );

    toast.promise(installPromise, {
      loading: $t("plugin_installing"),
      success: $t("plugin_installed"),
      error: toastErrorMessage($t("plugin_install_error")),
    });
  }

  function handleUninstall() {
    const revokePromise = uninstallPlugin(item.id);

    toast.promise(revokePromise, {
      loading: $t("plugin_uninstalling"),
      success: $t("plugin_uninstalled"),
      error: toastErrorMessage($t("plugin_uninstall_error")),
    });
  }

  async function handleUpdate(
    manifest: PluginManifest,
    remotePlugin: PluginRegistryEntry,
  ) {
    const updatePromise = update(manifest, remotePlugin);
    toast.promise(updatePromise, {
      loading: $t("plugin_updating"),
      success: $t("plugin_updated"),
      error: toastErrorMessage($t("plugin_update_error")),
    });
  }

  async function update(
    manifest: PluginManifest,
    remotePlugin: PluginRegistryEntry,
  ) {
    // Download the new bundle
    const bundle = await getPluginBundle(
      remotePlugin.repo,
      manifest.plugin.version,
    );

    // Uninstall the current plugin
    await uninstallPlugin(manifest.plugin.id);

    // Install the new version
    await installPluginBuffer(bundle);
  }
</script>

<div class="container">
  <div class="toolbar">
    {#if $manifestQuery.isLoading}
      <SkeletonList style="padding: 1rem" />
    {:else if $manifestQuery.isError}
      <Aside severity="error" style="margin: 1rem;">
        {$t("manifest_error", {
          values: { error: getErrorMessage($manifestQuery.error) },
        })}
      </Aside>
    {:else if $manifestQuery.isSuccess}
      <h2>{$manifestQuery.data.plugin.name}</h2>
      <p>{$manifestQuery.data.plugin.description}</p>
      <span>
        {$t("version")}: {$manifestQuery.data.plugin.version}

        {#if installed}
          <span class="installed-version">
            ({$t("installed")}: {installed.plugin.version})
          </span>
        {/if}
      </span>

      {#if installed}
        <div class="actions">
          {#if semverCompare($manifestQuery.data.plugin.version, installed.plugin.version) === 1}
            <Button onclick={() => handleUpdate($manifestQuery.data, item)}>
              {$t("update")}
            </Button>
          {/if}

          <Button onclick={handleUninstall}>{$t("uninstall")}</Button>
        </div>
      {:else}
        <Button disabled={$install.isPending} onclick={onInstall}>
          {$t("install")}
        </Button>
      {/if}
    {/if}
  </div>

  <div class="readme">
    {#if $readmeQuery.isLoading}
      <SkeletonList style="padding: 1rem" />
    {:else if $readmeQuery.isError}
      {$t("readme_error", {
        values: { error: getErrorMessage($readmeQuery.error) },
      })}
    {:else if $readmeQuery.isSuccess}
      {@const markdown = replaceMarkdownRelativeUrls(
        $readmeQuery.data.readme,
        $readmeQuery.data.baseURL,
      )}
      <Markdown source={markdown} />
    {/if}
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-flow: column;
    overflow: hidden;
    height: 100%;
  }

  .readme {
    flex: auto;
    overflow: hidden;
  }

  .toolbar {
    display: flex;
    flex-flow: column;
    width: 100%;
    background-color: #322e38;
    padding: 1rem;
    gap: 0.5rem;
  }

  .actions {
    display: flex;
    gap: 1rem;
  }

  .installed-version {
    color: #999;
  }
</style>
