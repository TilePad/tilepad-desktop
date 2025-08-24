<script lang="ts">
  import { Tooltip } from "bits-ui";
  import { queryClient } from "$lib/api/client";
  import RootLayout from "$lib/layouts/RootLayout.svelte";
  import Header from "$lib/components/layout/Header.svelte";
  import AppToaster from "$lib/components/AppToaster.svelte";
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import { serverContext } from "$lib/contexts/server.context";
  import { createI18n, i18nContext } from "$lib/i18n/i18n.svelte";
  import SettingsLoader from "$lib/components/SettingsLoader.svelte";
  import I18nProvider from "$lib/components/i18n/I18nProvider.svelte";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";
  import SettingsProvider from "$lib/components/SettingsProvider.svelte";
  import DeviceRequests from "$lib/components/devices/DeviceRequests.svelte";
  import UpdateNotification from "$lib/components/UpdateNotification.svelte";

  import type { LayoutProps } from "./$types";

  const { children: layoutChildren, data }: LayoutProps = $props();
  const port = $derived(data.port);

  const i18n = createI18n();
  i18nContext.set(i18n);

  serverContext.set({
    get serverURL() {
      return `http://127.0.0.1:${port}/`;
    },
  });
</script>

<RootLayout>
  <Tooltip.Provider>
    <QueryClientProvider client={queryClient}>
      <SettingsLoader>
        {#snippet children({ settings })}
          <SettingsProvider {settings}>
            <I18nProvider locale={settings.language}>
              <div class="layout">
                <Header />

                <main class="main">
                  {@render layoutChildren()}

                  <DeviceRequests />
                </main>
              </div>

              <AppToaster />
              <UpdateNotification />
            </I18nProvider>
          </SettingsProvider>
        {/snippet}
      </SettingsLoader>

      <SvelteQueryDevtools buttonPosition="bottom-left" position="bottom" />
    </QueryClientProvider>
  </Tooltip.Provider>
</RootLayout>

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
