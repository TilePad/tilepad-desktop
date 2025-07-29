<script lang="ts">
  import { getPluginAssetPath } from "$lib/api/utils/url";
  import { SvelteURLSearchParams } from "svelte/reactivity";
  import {
    type DisplayContext,
    encodeDisplayContext,
  } from "$lib/api/types/plugin";

  import type { DisplayMessage } from "./displayMessage";

  import { getServerContext } from "../ServerProvider.svelte";

  type Props = {
    ctx: DisplayContext;
    inspector: string;
    onFrameEvent: (
      ctx: DisplayContext,
      event: DisplayMessage,
      send: (data: object) => void,
    ) => void;
    onFrameMount: (ctx: DisplayContext, send: (data: object) => void) => void;
  };

  const { ctx, inspector, onFrameEvent, onFrameMount }: Props = $props();
  const serverContext = getServerContext();

  const inspectorSrc = $derived.by(() => {
    const params = new SvelteURLSearchParams();
    params.append("ctx", encodeDisplayContext(ctx));

    const baseSrc = getPluginAssetPath(
      serverContext.serverURL,
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

<iframe class="frame" bind:this={iframe} title="Inspector" src={inspectorSrc}>
</iframe>

<style>
  .frame {
    border: none;
    width: 100%;
    height: 100%;
  }
</style>
