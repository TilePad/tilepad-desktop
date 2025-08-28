<script lang="ts">
  import type { IconPackManifest } from "$lib/api/types/icons";
  import type { IconRegistryEntry } from "$lib/api/types/icons_registry";

  import { toast } from "svelte-sonner";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { createUninstallIconPackMutation } from "$lib/api/icons";
  import { replaceMarkdownRelativeUrls } from "$lib/utils/markdown";
  import { getErrorMessage, toastErrorMessage } from "$lib/api/utils/error";
  import {
    createIconPackReadmeQuery,
    createIconPackManifestQuery,
    createInstallIconPackFromRegistry,
  } from "$lib/api/icons_registry";

  import Aside from "../Aside.svelte";
  import Button from "../input/Button.svelte";
  import Markdown from "../markdown/Markdown.svelte";
  import SkeletonList from "../skeleton/SkeletonList.svelte";

  type Props = {
    item: IconRegistryEntry;
    installed?: IconPackManifest;
  };

  const { item, installed }: Props = $props();

  const i18n = i18nContext.get();

  const manifestQuery = createIconPackManifestQuery(() => item.repo);
  const readmeQuery = createIconPackReadmeQuery(() => item.repo);

  const install = createInstallIconPackFromRegistry();
  const uninstallMutation = createUninstallIconPackMutation();

  async function onInstall() {
    const manifest = manifestQuery.data;
    if (!manifest) return;

    const installPromise = install.mutateAsync(
      {
        repo: item.repo,
        version: manifest.version,
        fileName: item.fileName,
      },
      {},
    );

    toast.promise(installPromise, {
      loading: i18n.f("icon_packs_installing"),
      success: i18n.f("icon_packs_installed"),
      error: toastErrorMessage(i18n.f("icon_packs_install_error")),
    });
  }

  function handleUninstall() {
    const uninstallPromise = uninstallMutation.mutateAsync({
      packId: item.id,
    });

    toast.promise(uninstallPromise, {
      loading: i18n.f("icon_packs_uninstalling"),
      success: i18n.f("icon_packs_uninstalled"),
      error: toastErrorMessage(i18n.f("icon_packs_uninstall_error")),
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
      {@const manifest = manifestQuery.data}
      <div class="head">
        <div>
          <h2>{item.name}</h2>
          <p>{item.description}</p>
          <span>
            {i18n.f("version")}: {manifest.version}

            {#if installed}
              <span class="installed-version">
                ({i18n.f("installed")}: {installed.icons.version})
              </span>
            {/if}
          </span>
        </div>
      </div>

      <div class="actions">
        {#if installed !== undefined}
          <Button variant="error" onclick={handleUninstall}
            >{i18n.f("uninstall")}</Button
          >
        {:else}
          <Button disabled={install.isPending} onclick={onInstall}>
            {i18n.f("install")}
          </Button>
        {/if}
      </div>
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

  .head {
    display: flex;
    flex-flow: row;
    gap: var(--tp-space-2);
    align-items: flex-start;
    justify-content: space-between;
    align-items: center;
  }

  .readme {
    flex: auto;
    overflow: hidden;
  }

  .toolbar {
    display: flex;
    flex-flow: column;
    width: 100%;
    background-color: var(--tp-bg-secondary);
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
