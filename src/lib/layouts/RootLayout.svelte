<script lang="ts">
  import "$lib/styles/app.css";
  import "@fontsource-variable/roboto";
  import type { Snippet } from "svelte";

  type Props = {
    children?: Snippet;
  };

  const { children }: Props = $props();
</script>

{@render children?.()}
