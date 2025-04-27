<script lang="ts">
  import type { TileId } from "$lib/api/types/tiles";

  import { t } from "svelte-i18n";
  import { createTileQuery } from "$lib/api/tiles";
  import { createActionQuery } from "$lib/api/actions";
  import { getErrorMessage } from "$lib/api/utils/error";
  import SolarCloseCircleBold from "~icons/solar/close-circle-bold";

  import Aside from "../Aside.svelte";
  import Button from "../input/Button.svelte";
  import TileIconEditor from "./TileIconEditor.svelte";
  import TileNameEditor from "./TileNameEditor.svelte";
  import DeleteTileDialog from "./DeleteTileDialog.svelte";
  import { getFolderContext } from "../folders/FolderProvider.svelte";
  import PropertyInspector from "../property/PropertyInspector.svelte";
  import { getProfileContext } from "../profiles/ProfilesProvider.svelte";

  type Props = {
    tileId: TileId;
    onClose: VoidFunction;
  };

  const { tileId, onClose }: Props = $props();

  const { profile } = getProfileContext();
  const { folder } = getFolderContext();

  const currentProfile = $derived.by(profile);
  const currentFolder = $derived.by(folder);

  const tileQuery = createTileQuery(
    () => currentFolder.id,
    () => tileId,
  );

  const tile = $derived($tileQuery.data);

  const actionQuery = createActionQuery(
    () => tile?.plugin_id ?? null,
    () => tile?.action_id ?? null,
  );
</script>

{#if $tileQuery.isSuccess && $tileQuery.data}
  {@const tile = $tileQuery.data}
  {@const ctx = {
    profile_id: currentProfile.id,
    folder_id: tile.folder_id,
    plugin_id: tile.plugin_id,
    action_id: tile.action_id,
    tile_id: tile.id,
  }}

  <div class="header">
    <div>
      {#if $actionQuery.isSuccess && $actionQuery.data}
        {@const action = $actionQuery.data}
        {#if action.inspector !== null}
          <p class="titlebar__name">
            <b>{action.category_label}</b>: {action.label}
          </p>
        {/if}
      {/if}
    </div>

    <div class="header-actions">
      <DeleteTileDialog {tile} {onClose} />

      <Button transparent onclick={onClose}>
        <SolarCloseCircleBold width={24} height={24} />
      </Button>
    </div>
  </div>

  <div class="areas">
    <div class="tile-area">
      <TileIconEditor
        config={tile.config}
        action={$actionQuery.data ?? undefined}
        tileId={tile.id}
      />
      <TileNameEditor config={tile.config} tileId={tile.id} />
    </div>

    <div class="action-area">
      {#if $actionQuery.isSuccess && $actionQuery.data}
        {@const action = $actionQuery.data}
        {#if action.inspector !== null}
          <div class="inspector">
            <PropertyInspector {ctx} inspector={action.inspector} />
          </div>
        {/if}
      {:else if $actionQuery.isSuccess && $actionQuery.data === null}
        <Aside severity="error" title="Action not found" style="margin: 1rem;">
          <!-- eslint-disable-next-line svelte/no-at-html-tags -->
          {@html $t("action_not_found", {
            values: {
              plugin_id: tile.plugin_id,
              action_id: tile.action_id,
            },
          })}
        </Aside>
      {:else if $actionQuery.isLoading}
        <div class="skeleton-list" style="padding: 1rem;">
          <div class="skeleton" style="width: 80%; height: 1rem"></div>
          <div
            class="skeleton"
            style="width: 70%; height: 0.75rem; opacity: 0.5"
          ></div>
          <div
            class="skeleton"
            style="width: 30%; height: 0.75rem; opacity: 0.5"
          ></div>
        </div>
      {:else if $actionQuery.isError}
        <Aside severity="error" style="margin: 1rem;">
          {$t("action_error", {
            values: { error: getErrorMessage($actionQuery.error) },
          })}
        </Aside>
      {/if}
    </div>
  </div>
{:else if $tileQuery.isSuccess}
  <Aside severity="error" style="margin: 1rem;">
    {$t("tile_not_found")}
  </Aside>
{:else if $tileQuery.isLoading}
  <div class="skeleton-list" style="padding: 1rem;">
    <div class="skeleton" style="width: 120px; height: 120px"></div>

    <div class="skeleton" style="width: 80%; height: 1rem"></div>
    <div
      class="skeleton"
      style="width: 70%; height: 0.75rem; opacity: 0.5"
    ></div>
    <div
      class="skeleton"
      style="width: 30%; height: 0.75rem; opacity: 0.5"
    ></div>
  </div>
{:else if $tileQuery.isError}
  <Aside severity="error" style="margin: 1rem;">
    {$t("tile_error", {
      values: { error: getErrorMessage($tileQuery.error) },
    })}
  </Aside>
{/if}

<style>
  .areas {
    display: flex;
    flex-flow: column;
    flex: auto;

    width: 100%;
    overflow: hidden;
  }

  .tile-area {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
  }

  .action-area {
    display: flex;
    flex-flow: column;
    flex: auto;

    width: 100%;
    overflow: hidden;
  }

  .header {
    display: flex;
    justify-content: space-between;
    width: 100%;

    align-items: center;
    padding: 0.15rem 0.15rem;
    padding-left: 0.75rem;
    background-color: #151318;
    border-bottom: 2px solid #302d36;
  }

  .header-actions {
    display: flex;
  }

  .tile-area {
    background-color: #211e24;
    border-right: 2px solid #302d36;
    overflow-x: hidden;
    overflow-y: auto;
    padding: 0.5rem;
    display: flex;
    flex-flow: row;
    align-items: flex-start;
    gap: 0.65rem;
  }

  .inspector {
    flex: auto;
    width: 100%;
    overflow: hidden;
    position: relative;
  }
</style>
