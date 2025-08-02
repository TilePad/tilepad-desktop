<script lang="ts">
  import { getPluginAssetPath } from "$lib/api/utils/url";
  import { SvelteURLSearchParams } from "svelte/reactivity";
  import { openPluginInspector, closePluginInspector } from "$lib/api/plugins";
  import {
    type InspectorContext,
    encodeInspectorContext,
  } from "$lib/api/types/plugin";

  import type { PropertyInspectorMessage } from "./propertyInspectorMessage";

  import { getServerContext } from "../ServerProvider.svelte";

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

  const { ctx, inspector, onFrameEvent, onFrameMount }: Props = $props();
  const serverContext = getServerContext();

  const inspectorSrc = $derived.by(() => {
    const params = new SvelteURLSearchParams();
    params.append("ctx", encodeInspectorContext(ctx));

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
    openPluginInspector(ctxSnap);

    return () => {
      closePluginInspector(ctxSnap);
    };
  });
</script>

<svelte:window onmessage={onMessage} />

<iframe class="frame" bind:this={iframe} title="Inspector" src={inspectorSrc}
></iframe>

<style>
  .frame {
    border: none;
    width: 100%;
    height: 100%;
  }
</style>
