<script lang="ts">
  import "$lib/styles/app.css";

  import { QueryClientProvider } from "@tanstack/svelte-query";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";

  import { queryClient } from "$lib/api/client";
  import AppToaster from "$lib/components/AppToaster.svelte";

  let { children } = $props();
</script>

<QueryClientProvider client={queryClient}>
  <main>
    {@render children()}
  </main>

  <AppToaster />
  <SvelteQueryDevtools />
</QueryClientProvider>
