<script lang="ts">
  import "$lib/styles/app.css";
  import { queryClient } from "$lib/api/client";
  import Header from "$lib/components/layout/Header.svelte";
  import AppToaster from "$lib/components/AppToaster.svelte";
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";
  import FolderProvider from "$lib/components/folders/FolderProvider.svelte";
  import ProfilesProvider from "$lib/components/profiles/ProfilesProvider.svelte";

  let { children } = $props();
</script>

<QueryClientProvider client={queryClient}>
  <div class="layout">
    <Header />

    <main class="main">
      <ProfilesProvider>
        <FolderProvider>
          {@render children()}
        </FolderProvider>
      </ProfilesProvider>
    </main>
  </div>

  <AppToaster />
  <SvelteQueryDevtools buttonPosition="bottom-left" position="bottom" />
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
