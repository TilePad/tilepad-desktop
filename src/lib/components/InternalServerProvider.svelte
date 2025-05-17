<script lang="ts">
  import type { Snippet } from "svelte";

  import ServerProvider from "./ServerProvider.svelte";

  type Props = {
    children?: Snippet;
  };

  const { children }: Props = $props();

  const BACKEND_URL_PROD = "http://localhost:8532/";
  const BACKEND_URL_DEV = "http://localhost:8532/";

  const serverURL = import.meta.env.DEV ? BACKEND_URL_DEV : BACKEND_URL_PROD;
</script>

<ServerProvider {serverURL} {children} />
