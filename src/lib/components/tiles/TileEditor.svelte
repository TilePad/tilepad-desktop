<script lang="ts">
  import type { TileId } from "$lib/api/types/tiles";

  import { watch } from "runed";
  import { toast } from "svelte-sonner";
  import { createActionQuery } from "$lib/api/actions";
  import { sendPluginMessage } from "$lib/api/plugins";
  import { updateTile, deleteTile, createTileQuery } from "$lib/api/tiles";
  import { getErrorMessage, toastErrorMessage } from "$lib/api/utils/error";
  import SolarTrashBinTrashBoldDuotone from "~icons/solar/trash-bin-trash-bold-duotone";

  import Button from "../input/Button.svelte";
  import TileNameEditor from "./TileNameEditor.svelte";
  import TileIconEditor from "./TileIconEditor.svelte";
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

  const currentFolder = $derived.by(folder);
  const currentProfile = $derived.by(profile);

  const tileQuery = createTileQuery(
    () => currentFolder.id,
    () => tileId,
  );

  const tile = $derived($tileQuery.data);
  const tileConfig = $derived(tile?.config);

  const actionQuery = createActionQuery(
    () => tileConfig?.plugin_id ?? null,
    () => tileConfig?.action_id ?? null,
  );

  const action = $derived($actionQuery.data);

  watch(
    () => $tileQuery.data,
    (tile) => {
      if (tile === null) onClose();
    },
  );

  function onRemove() {
    const deletePromise = deleteTile(currentFolder.id, tileId);

    toast.promise(deletePromise, {
      loading: "Deleting tile",
      success: "Deleted tile",
      error: toastErrorMessage("Failed to delete tile"),
    });

    onClose();
  }
</script>

<div class="editor">
  {#if $tileQuery.isSuccess && $actionQuery.isSuccess && tile && action}
    <div class="titlebar">
      <p class="titlebar__name">
        <b>{action.category_label}</b>: {action.label}
      </p>

      <Button transparent variant="error" onclick={onRemove}>
        <SolarTrashBinTrashBoldDuotone width={24} height={24} />
      </Button>
    </div>

    <div class="content">
      <div class="left">
        <TileIconEditor icon={tile.config.icon} tileId={tile.id} />
        <TileNameEditor config={tile.config} tileId={tile.id} />
      </div>

      {#if action.inspector !== null}
        <div class="right">
          <PropertyInspector
            pluginId={action.plugin_id}
            tileId={tile.id}
            actionId={action.action_id}
            inspector={action.inspector}
            properties={tile.config.properties}
            onSetProperty={(name, value) => {
              updateTile(tile.id, {
                config: {
                  ...tile.config,
                  properties: {
                    ...tile.config.properties,
                    [name]: value,
                  },
                },
              });
            }}
            onSendPluginMessage={(message) => {
              sendPluginMessage(
                {
                  profile_id: currentProfile.id,
                  folder_id: currentFolder.id,
                  plugin_id: action.plugin_id,
                  action_id: action.action_id,
                  tile_id: tile.id,
                },
                message,
              );
            }}
          />
        </div>
      {/if}
    </div>
  {:else if $tileQuery.isError}
    Failed to load tile: {getErrorMessage($tileQuery.error)}
  {:else if $actionQuery.isError}
    Failed to load action: {getErrorMessage($actionQuery.error)}
  {:else if $tileQuery.isLoading}
    Loading tile...
  {:else if $actionQuery.isLoading}
    Loading action...
  {/if}
</div>

<style>
  .editor {
    position: relative;
    height: 35%;
    max-height: 250px;
    background-color: #28252c;
    flex: auto;
    width: 100%;
    border-top: 2px solid #302d36;
    flex-grow: 0;
    flex-shrink: 0;
    flex: auto;

    display: flex;
    flex-flow: column;
  }

  .content {
    display: flex;
    flex-flow: row;
    gap: 0.5rem;
    flex: auto;
    overflow: hidden;
  }

  .titlebar {
    display: flex;
    align-items: center;
    padding: 0.15rem 0.15rem;
    padding-left: 0.75rem;
    background-color: #151318;
    border-bottom: 2px solid #302d36;
  }

  .titlebar__name {
    flex: auto;
  }

  .left {
    background-color: #211e24;
    border-right: 2px solid #302d36;
    overflow-x: hidden;
    overflow-y: auto;
    padding: 0.5rem;
    display: flex;
    flex-flow: column;
    gap: 0.65rem;
  }

  .right {
    display: flex;
    flex: auto;
    flex-shrink: 0;
    overflow: hidden;
    position: relative;
  }
</style>
