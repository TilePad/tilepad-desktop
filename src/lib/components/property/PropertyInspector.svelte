<script lang="ts">
  import type { PluginId, PluginMessageContext } from "$lib/api/types/plugin";

  import { onMount, onDestroy } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { createPluginAssetTextQuery } from "$lib/api/plugins";

  import propertyInspectorScript from "./propertyInspectorScript?raw";

  type Props = {
    pluginId: PluginId;
    inspector: string;
    properties: object;

    onSendPluginMessage: (message: string) => void;
    onSetProperty: (name: string, value: unknown) => void;
  };
  const {
    pluginId,
    inspector,
    properties,
    onSendPluginMessage,
    onSetProperty,
  }: Props = $props();

  const assetQuery = createPluginAssetTextQuery(
    () => pluginId,
    () => inspector,
  );

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
    iframe.contentWindow.postMessage(data);
  }

  /**
   * Injects the property inspector into the provided HTML
   */
  function injectPropertyInspector(html: string) {
    return html.replace(
      "</head>",
      // eslint-disable-next-line no-useless-escape
      `<script>${propertyInspectorScript}<\/script></head>`,
    );
  }

  let unlisten: (() => void) | undefined;

  onMount(async () => {
    type Payload = {
      context: PluginMessageContext;
      message: unknown;
    };

    unlisten = await listen<Payload>("plugin:recv_plugin_message", (event) => {
      const { context, message } = event.payload;

      if (context.plugin_id !== pluginId) return;

      sendFrameEvent({
        type: "PLUGIN_MESSAGE",
        context,
        message,
      });
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
    unlisten = undefined;
  });
</script>

<svelte:window onmessage={onFrameEvent} />

{#if $assetQuery.isLoading}
  Loading...
{:else if $assetQuery.isError}
  Failed to load inspector: {getErrorMessage($assetQuery.error)}
{:else if $assetQuery.isSuccess}
  <iframe
    class="frame"
    bind:this={iframe}
    title="Inspector"
    srcdoc={injectPropertyInspector($assetQuery.data)}
  ></iframe>
{/if}

<style>
  .frame {
    border: none;
  }
</style>
