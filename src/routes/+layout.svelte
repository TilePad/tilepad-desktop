<script lang="ts">
  import "$lib/styles/app.css";

  import { QueryClientProvider } from "@tanstack/svelte-query";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";

  import { queryClient } from "$lib/api/client";
  import AppToaster from "$lib/components/AppToaster.svelte";
  import Header from "$lib/components/layout/Header.svelte";
  import ProfilesProvider from "$lib/components/profiles/ProfilesProvider.svelte";

  let { children } = $props();
</script>

<QueryClientProvider client={queryClient}>
  <ProfilesProvider>
    <div class="layout">
      <Header />

      <main class="main">
        {@render children()}
      </main>
    </div>
  </ProfilesProvider>

  <AppToaster />
  <SvelteQueryDevtools />
</QueryClientProvider>

<style>
  .layout {
    display: flex;
    flex-flow: column;
    height: 100%;
  }

  .main {
    flex: auto;
    position: relative;
    overflow: hidden;
  }
</style>
