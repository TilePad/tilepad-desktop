<script lang="ts">
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { uninstallPlugin } from "$lib/api/plugins";
  import { replaceMarkdownRelativeUrls } from "$lib/utils/markdown";
  import { getErrorMessage, toastErrorMessage } from "$lib/api/utils/error";
  import {
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
    installed: boolean;
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
      <span>{$t("version")}: {$manifestQuery.data.plugin.version}</span>

      {#if installed}
        <Button onclick={handleUninstall}>{$t("uninstall")}</Button>
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
</style>
