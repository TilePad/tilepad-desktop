<script lang="ts">
  import type { TileIcon, TileLabel } from "$lib/api/types/tiles";

  import {
    type InspectorContext,
    isInspectorContextEqual,
  } from "$lib/api/types/plugin";

  import PluginMessageListener from "./PluginMessageListener.svelte";
  import PropertyInspectorFrame from "./PropertyInspectorFrame.svelte";

  type Props = {
    ctx: InspectorContext;
    properties: object;

    inspector: string;

    onSendPluginMessage: (ctx: InspectorContext, message: string) => void;
    onSetProperties: (properties: Record<string, unknown>) => void;
    onSetIcon: (icon: TileIcon) => void;
    onSetLabel: (label: TileLabel) => void;
    onGetPluginProperties: (
      ctx: InspectorContext,
      callback: (properties: object) => void,
    ) => void;
    onSetPluginProperties: (
      ctx: InspectorContext,
      properties: object,
      partial: boolean,
    ) => void;
  };

  const {
    ctx,
    inspector,
    properties,
    onSendPluginMessage,
    onSetProperties,
    onGetPluginProperties,
    onSetPluginProperties,
    onSetIcon,
    onSetLabel,
  }: Props = $props();

  type CurrentFrameData = {
    ctx: InspectorContext;
    send: (data: object) => void;
  };

  let currentFrame: CurrentFrameData | undefined;

  function onFrameMount(ctx: InspectorContext, send: (data: object) => void) {
    currentFrame = { ctx, send };
  }

  function onMessage(ctx: InspectorContext, message: object) {
    if (currentFrame && isInspectorContextEqual(ctx, currentFrame.ctx)) {
      currentFrame.send({
        type: "PLUGIN_MESSAGE",
        context: ctx,
        message,
      });
    }
  }

  function onFrameEvent(
    ctx: InspectorContext,
    event: MessageEvent,
    send: (data: object) => void,
  ) {
    const data = event.data;
    const type = data.type;

    switch (type) {
      case "SEND_TO_PLUGIN": {
        onSendPluginMessage(ctx, data.message);
        break;
      }

      case "GET_TILE": {
        send({
          type: "TILE",
          tile: {
            profileId: ctx.profile_id,
            folderId: ctx.folder_id,
            pluginId: ctx.plugin_id,
            tileId: ctx.tile_id,
            actionId: ctx.action_id,
            properties,
          },
        });
        break;
      }

      case "GET_PROPERTIES": {
        send({
          type: "PROPERTIES",
          properties,
        });
        break;
      }

      case "SET_PROPERTIES": {
        onSetProperties(data.properties);
        break;
      }

      case "GET_PLUGIN_PROPERTIES": {
        onGetPluginProperties(ctx, (properties) => {
          send({
            type: "PLUGIN_PROPERTIES",
            properties,
          });
        });
        break;
      }

      case "SET_PLUGIN_PROPERTIES": {
        onSetPluginProperties(ctx, data.properties, data.partial);
        break;
      }

      case "SET_LABEL": {
        onSetLabel(data.label);
        break;
      }

      case "SET_ICON": {
        onSetIcon(data.icon);
        break;
      }
    }
  }
</script>

<PluginMessageListener {onMessage}>
  <PropertyInspectorFrame {onFrameEvent} {onFrameMount} {ctx} {inspector} />
</PluginMessageListener>
