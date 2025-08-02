<script lang="ts">
  import { Mutex } from "$lib/utils/mutex";
  import {
    UpdateKind,
    type TileIcon,
    type TileLabel,
  } from "$lib/api/types/tiles";
  import {
    type InspectorContext,
    isInspectorContextEqual,
  } from "$lib/api/types/plugin";
  import {
    sendPluginMessage,
    getPluginProperties,
    setPluginProperties,
  } from "$lib/api/plugins";
  import {
    getTile,
    createUpdateTileIconMutation,
    createUpdateTileLabelMutation,
    createUpdateTilePropertiesMutation,
  } from "$lib/api/tiles";

  import type { PropertyInspectorMessage } from "./propertyInspectorMessage";

  import PluginMessageListener from "./PluginMessageListener.svelte";
  import PropertyInspectorFrame from "./PropertyInspectorFrame.svelte";

  type Props = {
    ctx: InspectorContext;
    inspector: string;
  };

  const { ctx, inspector }: Props = $props();

  type CurrentFrameData = {
    ctx: InspectorContext;
    send: (data: object) => void;
  };

  const updateTileProperties = createUpdateTilePropertiesMutation();
  const updateTileLabel = createUpdateTileLabelMutation();
  const updateTileIcon = createUpdateTileIconMutation();

  /**
   * Mutex to ensure only one event is updating the tile state at a time
   * to ensure no data races and that the data in the updates is always
   * the latest
   */
  const updateMutex = new Mutex();

  let currentFrame: CurrentFrameData | undefined;

  /**
   * Handle the current frame mounting
   *
   * @param ctx
   * @param send
   */
  function onFrameMount(ctx: InspectorContext, send: (data: object) => void) {
    currentFrame = { ctx, send };
  }

  /**
   * Handle a message from the plugin to pass onto the inspector
   *
   * @param ctx
   * @param message
   */
  function onMessage(ctx: InspectorContext, message: object) {
    if (currentFrame && isInspectorContextEqual(ctx, currentFrame.ctx)) {
      currentFrame.send({
        type: "PLUGIN_MESSAGE",
        context: ctx,
        message,
      });
    }
  }

  /**
   * Handle a message from the inspector to pass onto the plugin
   *
   * @param ctx
   * @param message
   */
  function onSendToPlugin(ctx: InspectorContext, message: object) {
    sendPluginMessage(ctx, message);
  }

  async function onGetTile(
    ctx: InspectorContext,
    send: (data: object) => void,
  ) {
    const currentTile = await getTile(ctx.tile_id);

    // No tile active
    if (!currentTile) {
      return;
    }

    send({
      type: "TILE",
      tile: {
        profileId: ctx.profile_id,
        folderId: ctx.folder_id,
        pluginId: ctx.plugin_id,
        tileId: ctx.tile_id,
        actionId: ctx.action_id,
        properties: currentTile.properties,
      },
    });
  }

  async function onGetProperties(
    ctx: InspectorContext,
    send: (data: object) => void,
  ) {
    const currentTile = await getTile(ctx.tile_id);

    // No tile active
    if (!currentTile) {
      return;
    }

    send({
      type: "PROPERTIES",
      properties: currentTile.properties,
    });
  }

  function onSetProperties(ctx: InspectorContext, properties: object) {
    updateMutex.runExclusive(async () => {
      const currentTile = await getTile(ctx.tile_id);

      // No tile active
      if (!currentTile) {
        return;
      }

      await updateTileProperties.mutateAsync({
        tileId: ctx.tile_id,
        properties,
        partial: true,
      });
    });
  }

  function onSetLabel(ctx: InspectorContext, label: TileLabel) {
    updateMutex.runExclusive(async () => {
      const currentTile = await getTile(ctx.tile_id);

      // No tile active
      if (!currentTile) {
        return;
      }

      // User already has a label override
      if (currentTile.config.user_flags.label) {
        return;
      }

      await updateTileLabel.mutateAsync({
        tileId: currentTile.id,
        label,
        kind: UpdateKind.Program,
      });
    });
  }

  function onSetIcon(ctx: InspectorContext, icon: TileIcon) {
    updateMutex.runExclusive(async () => {
      const currentTile = await getTile(ctx.tile_id);

      // No tile active
      if (!currentTile) {
        return;
      }

      // User already has an icon override
      if (currentTile.config.user_flags.icon) {
        return;
      }

      await updateTileIcon.mutateAsync({
        tileId: currentTile.id,
        icon,
        kind: UpdateKind.Program,
      });
    });
  }

  async function onGetPluginProperties(
    ctx: InspectorContext,
    send: (data: object) => void,
  ) {
    const properties = await getPluginProperties(ctx.plugin_id);
    send({
      type: "PLUGIN_PROPERTIES",
      properties,
    });
  }

  async function onSetPluginProperties(
    ctx: InspectorContext,
    properties: object,
    partial: boolean = true,
  ) {
    await setPluginProperties(ctx.plugin_id, properties, partial);
  }

  function onFrameEvent(
    ctx: InspectorContext,
    event: PropertyInspectorMessage,
    send: (data: object) => void,
  ) {
    switch (event.type) {
      case "SEND_TO_PLUGIN": {
        onSendToPlugin(ctx, event.message);
        break;
      }

      case "GET_TILE": {
        onGetTile(ctx, send);
        break;
      }

      case "GET_PROPERTIES": {
        onGetProperties(ctx, send);
        break;
      }

      case "SET_PROPERTIES": {
        onSetProperties(ctx, event.properties);
        break;
      }

      case "GET_PLUGIN_PROPERTIES": {
        onGetPluginProperties(ctx, send);
        break;
      }

      case "SET_PLUGIN_PROPERTIES": {
        onSetPluginProperties(ctx, event.properties, event.partial);
        break;
      }

      case "SET_LABEL": {
        onSetLabel(ctx, event.label);
        break;
      }

      case "SET_ICON": {
        onSetIcon(ctx, event.icon);
        break;
      }
    }
  }
</script>

<PluginMessageListener {onMessage}>
  <PropertyInspectorFrame {onFrameEvent} {onFrameMount} {ctx} {inspector} />
</PluginMessageListener>
