<script lang="ts">
  import type { DisplayContext } from "$lib/api/types/plugin";

  import { onMount, type Snippet } from "svelte";
  import { listen } from "@tauri-apps/api/event";

  type Props = {
    onMessage: (context: DisplayContext, message: object) => void;
    children?: Snippet;
  };

  type Payload = {
    context: DisplayContext;
    message: object;
  };

  const { onMessage, children }: Props = $props();

  let loading = $state(true);

  onMount(() => {
    let mounted = true;
    let disposeFn: VoidFunction | undefined;

    listen<Payload>("plugin:recv_plugin_display_message", (event) => {
      const { context, message } = event.payload;
      onMessage(context, message);
    })
      // Handle listening complete
      .then((dispose) => {
        // Component is already unmounted (Dispose immediately)
        if (!mounted) {
          dispose();
          return;
        }

        // Set dispose function for later disposal
        disposeFn = dispose;
      })
      .finally(() => {
        loading = false;
      });

    return () => {
      mounted = false;
      if (disposeFn) disposeFn();
    };
  });
</script>

{#if loading}
  Loading...
{:else}
  {@render children?.()}
{/if}
