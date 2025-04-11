<script lang="ts">
  import type { TileIcon, TileLabel, TileConfig } from "$lib/api/types/tiles";

  import { watch } from "runed";
  import { onMount, onDestroy } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { getPluginAssetPath } from "$lib/api/utils/url";
  import { openPluginInspector, closePluginInspector } from "$lib/api/plugins";
  import {
    type InspectorContext,
    encodeInspectorContext,
    isInspectorContextEqual,
  } from "$lib/api/types/plugin";

  type Props = {
    ctx: InspectorContext;
    config: TileConfig;

    inspector: string;

    onSendPluginMessage: (ctx: InspectorContext, message: string) => void;
    onSetProperties: (properties: Record<string, unknown>) => void;
    onSetIcon: (icon: TileIcon) => void;
    onSetLabel: (label: TileLabel) => void;
  };

  const {
    ctx,
    inspector,
    config,
    onSendPluginMessage,
    onSetProperties,
    onSetIcon,
    onSetLabel,
  }: Props = $props();

  const properties = $derived(config.properties);

  let iframe: HTMLIFrameElement | undefined = $state(undefined);
  let currentCtx: InspectorContext | null = null;

  function onFrameEvent(event: MessageEvent) {
    if (!iframe) return;
    if (event.source !== iframe.contentWindow) return;

    const data = event.data;
    const type = data.type;

    switch (type) {
      case "SEND_TO_PLUGIN": {
        onSendPluginMessage(ctx, data.message);
        break;
      }

      case "GET_TILE": {
        sendFrameEvent({
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
        sendFrameEvent({
          type: "PROPERTIES",
          properties,
        });
        break;
      }

      case "SET_PROPERTIES": {
        onSetProperties(data.properties);
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

  function sendFrameEvent(data: object) {
    if (!iframe) return;
    if (!iframe.contentWindow) return;
    iframe.contentWindow.postMessage(data, "*");
  }

  let removeEventListener: (() => void) | undefined;

  onMount(async () => {
    type Payload = {
      context: InspectorContext;
      message: unknown;
    };

    removeEventListener = await listen<Payload>(
      "plugin:recv_plugin_message",
      (event) => {
        const { context, message } = event.payload;

        if (!isInspectorContextEqual(context, ctx)) return;

        sendFrameEvent({
          type: "PLUGIN_MESSAGE",
          context,
          message,
        });
      },
    );
  });

  onDestroy(() => {
    if (removeEventListener) removeEventListener();
    removeEventListener = undefined;

    if (currentCtx !== null) {
      console.debug("closed plugin inspector (destroy)");
      closePluginInspector(currentCtx);
      currentCtx = null;
    }
  });

  watch(
    () => ctx,
    (ctx) => {
      // Context has not changed
      if (currentCtx !== null && isInspectorContextEqual(ctx, currentCtx)) {
        return;
      }

      // Notify the previous inspector of closing
      if (currentCtx !== null) {
        console.debug("closed plugin inspector");
        closePluginInspector(currentCtx);
        currentCtx = null;
      }

      console.debug("opened plugin inspector");
      openPluginInspector(ctx);
      currentCtx = ctx;
    },
  );
</script>

<svelte:window onmessage={onFrameEvent} />

{#key encodeInspectorContext(ctx)}
  <iframe
    class="frame"
    bind:this={iframe}
    title="Inspector"
    src={getPluginAssetPath(ctx.plugin_id, inspector)}
  ></iframe>
{/key}

<style>
  .frame {
    border: none;
    width: 100%;
    height: 100%;
  }
</style>
