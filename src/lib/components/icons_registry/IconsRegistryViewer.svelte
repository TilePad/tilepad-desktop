<script lang="ts">
  import type { IconRegistryEntry } from "$lib/api/types/icons_registry";

  import { toast } from "svelte-sonner";
  import { uninstallIconPack } from "$lib/api/icons";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { replaceMarkdownRelativeUrls } from "$lib/utils/markdown";
  import {
    createIconPackReadmeQuery,
    createIconPackManifestQuery,
    createInstallIconPackFromRegistry,
  } from "$lib/api/icons_registry";

  import Button from "../input/Button.svelte";
  import Markdown from "../markdown/Markdown.svelte";

  type Props = {
    item: IconRegistryEntry;
    installed: boolean;
  };

  const { item, installed }: Props = $props();

  const manifestQuery = createIconPackManifestQuery(() => item.repo);
  const readmeQuery = createIconPackReadmeQuery(() => item.repo);

  const install = createInstallIconPackFromRegistry();

  async function onInstall() {
    const manifest = $manifestQuery.data;
    if (!manifest) return;

    const installPromise = $install.mutateAsync(
      {
        repo: item.repo,
        version: manifest.version,
        fileName: item.fileName,
      },
      {},
    );

    toast.promise(installPromise, {
      loading: "Installing icon pack...",
      success: "Installed icon pack",
      error: toastErrorMessage("Failed to install icon pack"),
    });
  }

  function handleUninstall() {
    const revokePromise = uninstallIconPack(item.id);

    toast.promise(revokePromise, {
      loading: "Uninstalling icon pack",
      success: "Uninstalled icon pack",
      error: toastErrorMessage("Failed to uninstall icon pack"),
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
      <h2>{item.name}</h2>
      <p>{item.description}</p>

      {#if installed}
        <Button onclick={handleUninstall}>Uninstall</Button>
      {:else}
        <Button disabled={$install.isPending} onclick={onInstall}
          >Install</Button
        >
      {/if}
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
