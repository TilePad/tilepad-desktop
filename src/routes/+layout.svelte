<script lang="ts">
  import "$lib/styles/app.css";
  import { queryClient } from "$lib/api/client";
  import Header from "$lib/components/layout/Header.svelte";
  import AppToaster from "$lib/components/AppToaster.svelte";
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";
  import FolderProvider from "$lib/components/profiles/FolderProvider.svelte";
  import ProfilesProvider from "$lib/components/profiles/ProfilesProvider.svelte";

  let { children } = $props();
</script>

<QueryClientProvider client={queryClient}>
  <ProfilesProvider>
    <FolderProvider>
      <div class="layout">
        <Header />

        <main class="main">
          {@render children()}
        </main>
      </div>
    </FolderProvider>
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
