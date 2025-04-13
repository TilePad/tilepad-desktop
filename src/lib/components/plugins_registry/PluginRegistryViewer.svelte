<script lang="ts">
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  import { toast } from "svelte-sonner";
  import SvelteMarkdown from "@humanspeak/svelte-markdown";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { replaceMarkdownRelativeUrls } from "$lib/utils/markdown";
  import {
    createPluginReadmeQuery,
    createPluginManifestQuery,
    createInstallPluginFromRegistry,
  } from "$lib/api/plugins_registry";

  import Button from "../input/Button.svelte";

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

{#if $manifestQuery.isLoading}
  Loading manifest..
{:else if $manifestQuery.isError}
  Failed to load manifest
{:else if $manifestQuery.isSuccess}
  Success: {JSON.stringify($manifestQuery.data)}

  <Button disabled={$install.isPending} onclick={onInstall}>Install</Button>
{/if}

{#if $readmeQuery.isLoading}
  Loading readme..
{:else if $readmeQuery.isError}
  Failed to load readme
{:else if $readmeQuery.isSuccess}
  <SvelteMarkdown
    source={replaceMarkdownRelativeUrls(
      $readmeQuery.data.readme,
      $readmeQuery.data.baseURL,
    )}
  />
{/if}
