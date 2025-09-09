<script lang="ts" module>
  import { Context } from "runed";

  interface PluginSearchContext {
    query: string;
  }

  export const pluginSearchContext = new Context<PluginSearchContext>(
    "PluginSearchContext",
  );
</script>

<script lang="ts">
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import SolarBoxBoldDuotone from "~icons/solar/box-bold-duotone";
  import SolarShopBoldDuotone from "~icons/solar/shop-bold-duotone";

  import type { LayoutProps } from "./$types";

  const { children }: LayoutProps = $props();

  const i18n = i18nContext.get();
  let search = $state("");

  pluginSearchContext.set({
    get query() {
      return search;
    },

    set query(value: string) {
      search = value;
    },
  });
</script>

<div class="layout">
  <div class="header">
    <div class="nav">
      <a class="tab" href="/plugins">
        <SolarBoxBoldDuotone />

        {i18n.f("installed")}
      </a>
      <a class="tab tab--active" href="/plugins/community">
        <SolarShopBoldDuotone />

        {i18n.f("community_plugins")}
      </a>
    </div>

    <input
      bind:value={search}
      class="search"
      type="text"
      placeholder={i18n.f("search_placeholder")}
    />
  </div>

  {@render children?.()}
</div>

<style>
  .layout {
    height: 100%;
    overflow: hidden;

    display: flex;
    flex-flow: column;
  }

  .header {
    padding: var(--tp-space-4);
    display: flex;
    gap: var(--tp-space-4);
    justify-content: space-between;
  }

  .search {
    padding: 0.5rem;
    background-color: #1f1d22;
    border: 1px solid #666;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    flex: auto;
  }

  .nav {
    display: flex;
    align-items: center;
    flex-shrink: 0;
    width: calc(24rem - var(--tp-space-4) * 2);
  }

  .tab {
    display: inline-flex;
    align-items: center;
    flex: auto;
    gap: var(--tp-space-2);
    justify-content: center;
    font-size: var(--tp-text-base);
    font-weight: var(--tp-font-weight-medium);
    text-decoration: none;
    color: var(--tp-text-primary);
    border-bottom: 2px solid transparent;
    height: var(--tp-btn-height-md);
    padding: 0 var(--tp-btn-padding-x-md);
    border-radius: var(--tp-radius-md);
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
  }

  .tab--active {
    background: var(--tp-bg-tertiary);
    border-bottom: 2px solid var(--tp-text-primary);
  }
</style>
