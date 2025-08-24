<script lang="ts">
  import type { PluginManifest } from "$lib/api/types/plugin";
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  import { toast } from "svelte-sonner";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { compare as semverCompare } from "semver-ts";
  import { createUninstallPlugin } from "$lib/api/plugins";
  import { replaceMarkdownRelativeUrls } from "$lib/utils/markdown";
  import { getErrorMessage, toastErrorMessage } from "$lib/api/utils/error";
  import {
    createUpdatePlugin,
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

  const i18n = i18nContext.get();

  const manifestQuery = createPluginManifestQuery(() => item.repo);
  const readmeQuery = createPluginReadmeQuery(() => item.repo);

  const install = createInstallPluginFromRegistry();
  const uninstall = createUninstallPlugin();
  const update = createUpdatePlugin();

  async function onInstall() {
    const manifest = manifestQuery.data;
    if (!manifest) return;

    const installPromise = install.mutateAsync(
      {
        repo: item.repo,
        version: manifest.plugin.version,
      },
      {},
    );

    toast.promise(installPromise, {
      loading: i18n.f("plugin_installing"),
      success: i18n.f("plugin_installed"),
      error: toastErrorMessage(i18n.f("plugin_install_error")),
    });
  }

  function handleUninstall() {
    const revokePromise = uninstall.mutateAsync({ pluginId: item.id });

    toast.promise(revokePromise, {
      loading: i18n.f("plugin_uninstalling"),
      success: i18n.f("plugin_uninstalled"),
      error: toastErrorMessage(i18n.f("plugin_uninstall_error")),
    });
  }

  async function handleUpdate(
    manifest: PluginManifest,
    remotePlugin: PluginRegistryEntry,
  ) {
    const updatePromise = update.mutateAsync({
      repo: remotePlugin.repo,
      version: manifest.plugin.version,
      pluginId: manifest.plugin.id,
    });

    toast.promise(updatePromise, {
      loading: i18n.f("plugin_updating"),
      success: i18n.f("plugin_updated"),
      error: toastErrorMessage(i18n.f("plugin_update_error")),
    });
  }
</script>

<div class="container">
  <div class="toolbar">
    {#if manifestQuery.isLoading}
      <SkeletonList style="padding: 1rem" />
    {:else if manifestQuery.isError}
      <Aside severity="error" style="margin: 1rem;">
        {i18n.f("manifest_error", {
          values: { error: getErrorMessage(manifestQuery.error) },
        })}
      </Aside>
    {:else if manifestQuery.isSuccess}
      <h2>{manifestQuery.data.plugin.name}</h2>
      <p>{manifestQuery.data.plugin.description}</p>
      <span>
        {i18n.f("version")}: {manifestQuery.data.plugin.version}

        {#if installed}
          <span class="installed-version">
            ({i18n.f("installed")}: {installed.plugin.version})
          </span>
        {/if}
      </span>

      {#if installed}
        <div class="actions">
          {#if semverCompare(manifestQuery.data.plugin.version, installed.plugin.version) === 1}
            <Button
              onclick={() => handleUpdate(manifestQuery.data, item)}
              loading={update.isPending}
              disabled={uninstall.isPaused}
            >
              {i18n.f("update")}
            </Button>
          {/if}

          <Button
            onclick={handleUninstall}
            loading={uninstall.isPending}
            disabled={update.isPending}>{i18n.f("uninstall")}</Button
          >
        </div>
      {:else}
        <Button loading={install.isPending} onclick={onInstall}>
          {install.isPending ? i18n.f("installing") : i18n.f("install")}
        </Button>
      {/if}
    {/if}
  </div>

  <div class="readme">
    {#if readmeQuery.isLoading}
      <SkeletonList style="padding: 1rem" />
    {:else if readmeQuery.isError}
      {i18n.f("readme_error", {
        values: { error: getErrorMessage(readmeQuery.error) },
      })}
    {:else if readmeQuery.isSuccess}
      {@const markdown = replaceMarkdownRelativeUrls(
        readmeQuery.data.readme,
        readmeQuery.data.baseURL,
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
