<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  import propertyInspectorScript from "./propertyInspectorScript?raw";

  let message = "Nothing";

  const channel = new BroadcastChannel("TILEPAD_INSPECTOR");

  function onFrameEvent(event: MessageEvent) {
    message = `Received from iframe: ${event.data}`;
  }

  function sendFrameEvent(data: string) {
    channel.postMessage(data);
  }

  onMount(() => {
    channel.onmessage = onFrameEvent;
  });

  onDestroy(() => {
    channel.close();
  });

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

<div>
  <button on:click={() => sendFrameEvent("Test")}>Send Message to Iframe</button
  >
  <p>{message}</p>

  <iframe title="Test" srcdoc={iframeContent}></iframe>
</div>
