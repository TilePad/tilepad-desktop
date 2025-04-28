<script lang="ts">
  import "$lib/styles/app.css";
  import "@fontsource-variable/roboto";
  import { Tooltip } from "bits-ui";
  import { queryClient } from "$lib/api/client";
  import Header from "$lib/components/layout/Header.svelte";
  import AppToaster from "$lib/components/AppToaster.svelte";
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import I18nProvider from "$lib/components/I18nProvider.svelte";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";
  import SettingsProvider from "$lib/components/SettingsProvider.svelte";
  import FolderProvider from "$lib/components/folders/FolderProvider.svelte";
  import DeviceRequests from "$lib/components/devices/DeviceRequests.svelte";
  import UpdateNotification from "$lib/components/UpdateNotification.svelte";
  import ProfilesProvider from "$lib/components/profiles/ProfilesProvider.svelte";
  import InternalServerProvider from "$lib/components/InternalServerProvider.svelte";

  let { children } = $props();
</script>

<I18nProvider>
  <Tooltip.Provider>
    <QueryClientProvider client={queryClient}>
      <InternalServerProvider>
        <div class="layout">
          <Header />

          <main class="main">
            <SettingsProvider>
              <ProfilesProvider>
                <FolderProvider>
                  {@render children()}

                  <DeviceRequests />
                </FolderProvider>
              </ProfilesProvider>
            </SettingsProvider>
          </main>
        </div>

        <AppToaster />
        <UpdateNotification />
      </InternalServerProvider>

      <SvelteQueryDevtools buttonPosition="bottom-left" position="bottom" />
    </QueryClientProvider>
  </Tooltip.Provider>
</I18nProvider>

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
