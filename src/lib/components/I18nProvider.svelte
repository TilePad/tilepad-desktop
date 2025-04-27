<!-- Setup and load i18n -->
<script lang="ts" module>
  import { init, register, getLocaleFromNavigator } from "svelte-i18n";

  // Register languages
  register("en", () => import("../i18n/en.json"));

  // Initialize i18n
  init({
    fallbackLocale: "en",
    initialLocale: getLocaleFromNavigator(),
  });
</script>

<script lang="ts">
  import type { Snippet } from "svelte";

  import { waitLocale } from "svelte-i18n";

  type Props = {
    children?: Snippet;
  };

  const { children }: Props = $props();
</script>

{#await waitLocale()}
  <!-- Loading current locale -->
  <div class="skeleton-list">
    <div class="skeleton" style="width: 80%; height: 1rem"></div>
    <div class="skeleton" style="width: 70%; height: 1rem"></div>
    <div class="skeleton" style="width: 30%; height: 1rem"></div>
  </div>
{:then}
  {@render children?.()}
{/await}
