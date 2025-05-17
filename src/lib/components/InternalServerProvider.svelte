<script lang="ts">
  import type { Snippet } from "svelte";

  import { getServerPort } from "$lib/api/server";

  import ServerProvider from "./ServerProvider.svelte";
  import SkeletonList from "./skeleton/SkeletonList.svelte";

  type Props = {
    children?: Snippet;
  };

  const { children }: Props = $props();
</script>

{#await getServerPort()}
  <SkeletonList />
{:then port}
  <ServerProvider serverURL="http://127.0.0.1:{port}/" {children} />
{/await}
