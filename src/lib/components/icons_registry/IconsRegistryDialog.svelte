<script lang="ts">
  import type { IconRegistryEntry } from "$lib/api/types/icons_registry";

  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { createIconPacksQuery } from "$lib/api/icons";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { createIconPackRegistryQuery } from "$lib/api/icons_registry";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Aside from "../Aside.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import IconsRegistryItem from "./IconsRegistryItem.svelte";
  import SkeletonList from "../skeleton/SkeletonList.svelte";
  import IconsRegistryViewer from "./IconsRegistryViewer.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & {};

  const { ...restProps }: Props = $props();

  const i18n = i18nContext.get();

  const iconRegistryQuery = createIconPackRegistryQuery();
  const iconPacksQuery = createIconPacksQuery();

  let active: IconRegistryEntry | undefined = $state(undefined);
  let search = $state("");

  const filteredRegistry = $derived(
    filterIconPacks(iconRegistryQuery.data ?? [], search),
  );

  function filterIconPacks(packs: IconRegistryEntry[], query: string) {
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
    {#if iconRegistryQuery.isLoading || iconPacksQuery.isLoading}
      <SkeletonList style="padding: 1rem" />
    {:else if iconRegistryQuery.isError}
      <Aside severity="error" style="margin: 1rem;">
        {i18n.f("community_icons_error", {
          values: { error: getErrorMessage(iconRegistryQuery.error) },
        })}
      </Aside>
    {:else if iconPacksQuery.isError}
      <Aside severity="error" style="margin: 1rem;">
        {i18n.f("icon_packs_installed_error", {
          values: { error: getErrorMessage(iconPacksQuery.error) },
        })}
      </Aside>
    {:else if iconRegistryQuery.isSuccess && iconPacksQuery.isSuccess}
      <div class="split">
        <div class="plugins">
          <div class="titlebar">
            <div class="titlebar__text">
              <h2>{i18n.f("community_icons")}</h2>
              <p class="total">
                {i18n.f("count_icon_packs", {
                  values: { count: filteredRegistry.length },
                })}
              </p>
            </div>

            <DialogCloseButton buttonLabel={{ text: i18n.f("close") }} />
          </div>

          <input
            bind:value={search}
            class="search"
            type="text"
            placeholder={i18n.f("search_placeholder")}
          />

          <div class="plugins-list">
            {#each filteredRegistry as item (item.id)}
              <IconsRegistryItem
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
                installed={iconPacksQuery.data.find(
                  (plugin) => plugin.manifest.icons.id === item.id,
                ) !== undefined}
                selected={active !== undefined && active.id === item.id}
              />
            {:else}
              {i18n.f("community_icons_none")}
            {/each}
          </div>
        </div>

        <div class="viewer">
          {#if active}
            <IconsRegistryViewer
              item={active}
              installed={iconPacksQuery.data.find(
                (plugin) => plugin.manifest.icons.id === active!.id,
              ) !== undefined}
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
    justify-content: space-between;
    align-items: center;
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
