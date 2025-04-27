<!-- Setup and load i18n -->
<script lang="ts" module>
  import { init, register, getLocaleFromNavigator } from "svelte-i18n";

  // Register languages
  register("en", () => import("../i18n/locales/en.json"));

  // Initialize i18n
  init({
    fallbackLocale: "en",
    initialLocale: getLocaleFromNavigator(),
  });
</script>

<script lang="ts">
  import type { Snippet } from "svelte";

  import { waitLocale } from "svelte-i18n";

  import SkeletonList from "./skeleton/SkeletonList.svelte";

  type Props = {
    children?: Snippet;
  };

  const { children }: Props = $props();
</script>

{#await waitLocale()}
  <!-- Loading current locale -->
  <SkeletonList />
{:then}
  {@render children?.()}
{/await}
