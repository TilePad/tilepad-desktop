<script lang="ts">
  import "$lib/styles/app.css";
  import "@fontsource/roboto/400.css";
  import "@fontsource/roboto/500.css";
  import "@fontsource/roboto/600.css";
  import "@fontsource/roboto/700.css";
  import type { Snippet } from "svelte";

  type Props = {
    children?: Snippet;
  };

  const { children }: Props = $props();
</script>

{@render children?.()}
