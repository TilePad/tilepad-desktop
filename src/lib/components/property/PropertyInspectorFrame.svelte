<script lang="ts">
  import { getPluginAssetPath } from "$lib/api/utils/url";
  import { SvelteURLSearchParams } from "svelte/reactivity";
  import { serverContext } from "$lib/contexts/server.context";
  import { openPluginInspector, closePluginInspector } from "$lib/api/plugins";
  import {
    type InspectorContext,
    encodeInspectorContext,
  } from "$lib/api/types/plugin";

  import type { PropertyInspectorMessage } from "./propertyInspectorMessage";

  import LoadingSpinner from "../loading/LoadingSpinner.svelte";

  type Props = {
    ctx: InspectorContext;
    inspector: string;
    onFrameEvent: (
      ctx: InspectorContext,
      event: PropertyInspectorMessage,
      send: (data: object) => void,
    ) => void;
    onFrameMount: (ctx: InspectorContext, send: (data: object) => void) => void;
  };

  let loading = $state(true);

  const { ctx, inspector, onFrameEvent, onFrameMount }: Props = $props();
  const currentServerContext = serverContext.get();

  const inspectorSrc = $derived.by(() => {
    const params = new SvelteURLSearchParams();
    params.append("ctx", encodeInspectorContext(ctx));

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
    openPluginInspector(ctxSnap);

    return () => {
      closePluginInspector(ctxSnap);
    };
  });
</script>

<svelte:window onmessage={onMessage} />

<div class="frame-container">
  <iframe
    class="frame"
    bind:this={iframe}
    title="Inspector"
    src={inspectorSrc}
    onload={() => (loading = false)}
  ></iframe>

  {#if loading}
    <div class="loading-container">
      <LoadingSpinner size={80} />
    </div>
  {/if}
</div>

<style>
  .frame,
  .frame-container {
    position: relative;
    border: none;
    width: 100%;
    height: 100%;
  }

  .loading-container {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    background-color: #1f1e22;
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>
