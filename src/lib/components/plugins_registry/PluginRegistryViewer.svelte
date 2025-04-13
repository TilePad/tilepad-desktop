<script lang="ts">
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { replaceMarkdownRelativeUrls } from "$lib/utils/markdown";
  import {
    createPluginReadmeQuery,
    createPluginManifestQuery,
    createInstallPluginFromRegistry,
  } from "$lib/api/plugins_registry";

  import Button from "../input/Button.svelte";
  import Markdown from "../markdown/Markdown.svelte";

  type Props = {
    item: PluginRegistryEntry;
  };

  const { item }: Props = $props();

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
      loading: "Installing plugin...",
      success: "Installed plugin",
      error: toastErrorMessage("Failed to install plugin"),
    });
  }
</script>

<div class="container">
  <div class="toolbar">
    {#if $manifestQuery.isLoading}
      Loading manifest..
    {:else if $manifestQuery.isError}
      Failed to load manifest
    {:else if $manifestQuery.isSuccess}
      <h2>{$manifestQuery.data.plugin.name}</h2>
      <p>{$manifestQuery.data.plugin.description}</p>
      <span>Version: {$manifestQuery.data.plugin.version}</span>

      <Button disabled={$install.isPending} onclick={onInstall}>Install</Button>
    {/if}
  </div>

  <div class="readme">
    {#if $readmeQuery.isLoading}
      Loading readme..
    {:else if $readmeQuery.isError}
      Failed to load readme
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
    overflow: auto;
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
