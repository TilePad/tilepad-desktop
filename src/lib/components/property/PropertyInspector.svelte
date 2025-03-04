<script lang="ts">
  import type { TileId } from "$lib/api/types/tiles";
  import type { PluginId, PluginMessageContext } from "$lib/api/types/plugin";

  import { watch } from "runed";
  import { onMount, onDestroy } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { getPluginAssetPath } from "$lib/api/utils/url";

  type Props = {
    pluginId: PluginId;
    tileId: TileId;

    inspector: string;
    properties: object;

    onSendPluginMessage: (message: string) => void;
    onSetProperty: (name: string, value: unknown) => void;
  };
  const {
    pluginId,
    tileId,
    inspector,
    properties,
    onSendPluginMessage,
    onSetProperty,
  }: Props = $props();

  let iframe: HTMLIFrameElement | undefined = $state(undefined);

  function onFrameEvent(event: MessageEvent) {
    if (!iframe) return;
    if (event.source !== iframe.contentWindow) return;

    const data = event.data;
    const type = data.type;

    switch (type) {
      case "SEND_TO_PLUGIN": {
        onSendPluginMessage(data.message);
        break;
      }

      case "GET_PROPERTIES": {
        sendFrameEvent({
          type: "PROPERTIES",
          properties,
        });
        break;
      }

      case "SET_PROPERTY": {
        onSetProperty(data.name, data.value);
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
      context: PluginMessageContext;
      message: unknown;
    };

    removeEventListener = await listen<Payload>(
      "plugin:recv_plugin_message",
      (event) => {
        const { context, message } = event.payload;

        if (context.plugin_id !== pluginId) return;

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
  });

  watch(
    () => tileId,
    () => {
      sendFrameEvent({
        type: "REFRESH",
      });
    },
  );
</script>

<svelte:window onmessage={onFrameEvent} />

<iframe
  class="frame"
  bind:this={iframe}
  title="Inspector"
  src={getPluginAssetPath(pluginId, inspector)}
></iframe>

<style>
  .frame {
    border: none;
    width: 100%;
    height: 100%;
  }
</style>
