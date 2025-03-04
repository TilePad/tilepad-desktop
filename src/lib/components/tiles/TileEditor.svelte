<script lang="ts">
  import type { TileId } from "$lib/api/types/tiles";

  import { watch } from "runed";
  import { createActionQuery } from "$lib/api/actions";
  import { sendPluginMessage } from "$lib/api/plugins";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { updateTile, createTileQuery } from "$lib/api/tiles";

  import TileNameEditor from "./TileNameEditor.svelte";
  import { getFolderContext } from "../folders/FolderProvider.svelte";
  import PropertyInspector from "../property/PropertyInspector.svelte";
  import { getProfileContext } from "../profiles/ProfilesProvider.svelte";
  import Button from "../input/Button.svelte";

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
</script>

<div class="editor">
  {#if $tileQuery.isSuccess && $actionQuery.isSuccess && tile && action}
    <div class="left">
      <Button onclick={onClose}>Close</Button>

      <TileNameEditor config={tile.config} tileId={tile.id} />
    </div>

    {#if action.inspector !== null}
      <div class="right">
        <PropertyInspector
          pluginId={action.plugin_id}
          tileId={tile.id}
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
    height: 250px;
    background-color: #28252c;
    flex: auto;
    width: 100%;
    border-top: 2px solid #302d36;
    flex-grow: 0;
    flex-shrink: 0;
    flex: auto;

    display: flex;
    flex-flow: row;
    gap: 0.5rem;
  }

  .left {
    overflow-x: hidden;
    overflow-y: auto;
  }

  .right {
    display: flex;
    flex: auto;
    flex-shrink: 0;
    overflow: hidden;
  }
</style>
