<script lang="ts">
  import { getPluginAssetPath } from "$lib/api/utils/url";
  import { serverContext } from "$lib/contexts/server.context";
  import {
    type DisplayContext,
    encodeDisplayContext,
  } from "$lib/api/types/plugin";

  import type { DisplayMessage } from "./displayMessage";

  type Props = {
    ctx: DisplayContext;
    inspector: string;
    style: string;
    onFrameEvent: (
      ctx: DisplayContext,
      event: DisplayMessage,
      send: (data: object) => void,
    ) => void;
    onFrameMount: (ctx: DisplayContext, send: (data: object) => void) => void;
  };

  const { ctx, inspector, style, onFrameEvent, onFrameMount }: Props = $props();
  const currentServerContext = serverContext.get();

  const inspectorSrc = $derived.by(() => {
    // eslint-disable-next-line svelte/prefer-svelte-reactivity
    const params = new URLSearchParams();
    params.append("ctx", encodeDisplayContext(ctx));

    const baseSrc = getPluginAssetPath(
      currentServerContext.serverURL,
      ctx.plugin_id,
      inspector,
    );

    return `${baseSrc}?${params.toString()}`;
  });

  let iframe: HTMLIFrameElement | undefined = $state(undefined);

  function send(data: object) {
    if (!iframe || !iframe.contentWindow) return;
    iframe.contentWindow.postMessage(data, "*");
  }

  function onMessage(event: MessageEvent) {
    if (!iframe || event.source !== iframe.contentWindow) return;
    onFrameEvent(ctx, event.data, send);
  }

  $effect(() => {
    let ctxSnap = $state.snapshot(ctx);
    onFrameMount(ctxSnap, send);
  });
</script>

<svelte:window onmessage={onMessage} />

<iframe
  class="frame"
  bind:this={iframe}
  title="Inspector"
  src={inspectorSrc}
  allowtransparency={true}
  {style}
>
</iframe>

<style>
  .frame {
    background: transparent;
    border: none;
    width: 100%;
    height: 100%;
  }
</style>
