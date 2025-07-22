<script lang="ts">
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  import { t } from "svelte-i18n";
  import { createPluginsQuery } from "$lib/api/plugins";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { createPluginRegistryQuery } from "$lib/api/plugins_registry";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Aside from "../Aside.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import SkeletonList from "../skeleton/SkeletonList.svelte";
  import PluginsRegistryItem from "./PluginsRegistryItem.svelte";
  import PluginRegistryViewer from "./PluginRegistryViewer.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & {};

  const { ...restProps }: Props = $props();

  const pluginRegistryQuery = createPluginRegistryQuery();
  const pluginsQuery = createPluginsQuery();

  let active: PluginRegistryEntry | undefined = $state(undefined);
  let search = $state("");

  const filteredRegistry = $derived(
    filterIconPacks($pluginRegistryQuery.data ?? [], search),
  );

  function filterIconPacks(packs: PluginRegistryEntry[], query: string) {
    query = query.toLowerCase();

    if (query.length < 1) return packs;

    return packs.filter((entry) => {
      const name = entry.name.toLowerCase();
      return name === query || name.includes(query);
    });
  }
</script>

<Dialog {...restProps}>
  <div class="content">
    {#if $pluginRegistryQuery.isLoading || $pluginsQuery.isLoading}
      <SkeletonList style="padding: 1rem" />
    {:else if $pluginRegistryQuery.isError}
      <Aside severity="error" style="margin: 1rem;">
        {$t("community_plugins_error", {
          values: { error: getErrorMessage($pluginRegistryQuery.error) },
        })}
      </Aside>
    {:else if $pluginsQuery.isError}
      <Aside severity="error" style="margin: 1rem;">
        {$t("plugins_installed_error", {
          values: { error: getErrorMessage($pluginsQuery.error) },
        })}
      </Aside>
    {:else if $pluginRegistryQuery.isSuccess && $pluginsQuery.isSuccess}
      <div class="split">
        <div class="plugins">
          <div class="titlebar">
            <div class="titlebar__text">
              <h2>{$t("community_plugins")}</h2>
              <p class="total">
                {$t("count_plugins", {
                  values: { count: filteredRegistry.length },
                })}
              </p>
            </div>

            <DialogCloseButton buttonLabel={{ text: $t("close") }} />
          </div>

          <input
            bind:value={search}
            class="search"
            type="text"
            placeholder={$t("search_placeholder")}
          />

          <div class="plugins-list">
            {#each filteredRegistry as item (item.id)}
              <PluginsRegistryItem
                name={item.name}
                authors={item.authors}
                description={item.description}
                repo={item.repo}
                onClick={() => {
                  if (active !== undefined && active.id === item.id) {
                    active = undefined;
                  } else {
                    active = item;
                  }
                }}
                installed={$pluginsQuery.data.find(
                  (plugin) => plugin.manifest.plugin.id === item.id,
                ) !== undefined}
                selected={active !== undefined && active.id === item.id}
              />
            {:else}
              {$t("community_plugins_none")}
            {/each}
          </div>
        </div>

        <div class="viewer">
          {#if active}
            <PluginRegistryViewer
              item={active}
              installed={$pluginsQuery.data.find(
                (plugin) => plugin.manifest.plugin.id === active!.id,
              )?.manifest}
            />
          {/if}
        </div>
      </div>
    {/if}
  </div>
</Dialog>

<style>
  .content {
    max-width: 100%;
    width: 90vw;
    height: 90vh;

    display: flex;
    flex-flow: column;
    padding-bottom: 0;
  }

  .split {
    display: flex;
    flex-flow: row;
    height: 100%;
    overflow: hidden;
  }

  .plugins {
    display: flex;
    flex-flow: column;

    height: 100%;
    overflow: auto;
    max-width: 20rem;
    flex: auto;
    flex-shrink: 0;
    padding: 1rem;
    gap: 0.5rem;
  }

  .viewer {
    height: 100%;
    overflow: auto;
    flex: auto;
  }

  .plugins-list {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.5rem;
    flex-direction: column;
    flex: auto;
    overflow: auto;
  }

  .titlebar {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    justify-content: space-between;
  }

  .total {
    font-size: 0.8rem;
  }

  .search {
    padding: 0.5rem;
    background-color: #1f1d22;
    border: 1px solid #666;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    width: 100%;
  }
</style>
