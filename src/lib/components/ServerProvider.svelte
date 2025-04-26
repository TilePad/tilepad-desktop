<script module lang="ts">
  import { getContext, setContext, type Snippet } from "svelte";

  const serverContextKey = Symbol("SERVER_CONTEXT");

  interface ServerProviderContext {
    serverURL: string;
  }

  export function getServerContext(): ServerProviderContext {
    return getContext(serverContextKey);
  }
</script>

<script lang="ts">
  type Props = {
    serverURL: string;
    children?: Snippet;
  };

  const { serverURL, children }: Props = $props();

  setContext(serverContextKey, {
    get serverURL() {
      return serverURL;
    },
  });
</script>

{@render children?.()}
