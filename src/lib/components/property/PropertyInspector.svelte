<script lang="ts">
  import type { PluginId } from "$lib/api/types/plugin";
  import propertyInspectorScript from "./propertyInspectorScript?raw";

  type Props = {
    pluginId: PluginId;
    inspector: string;
  };
  const { pluginId, inspector }: Props = $props();

  let message = $state("Nothing");
  let iframe: HTMLIFrameElement | undefined = $state(undefined);

  function onFrameEvent(event: MessageEvent) {
    if (!iframe) return;
    if (event.source !== iframe.contentWindow) return;
    message = `Received from iframe: ${event.data}`;
  }

  function sendFrameEvent(data: string) {
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

  const testDocument = `
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>

<body>
  <p>Content</p>
</body>

</html>
    `;
  const iframeContent = injectPropertyInspector(testDocument);
</script>

<svelte:window onmessage={onFrameEvent} />

<iframe class="frame" bind:this={iframe} title="Test" srcdoc={iframeContent}
></iframe>

<style>
  .frame {
    border: none;
  }
</style>
