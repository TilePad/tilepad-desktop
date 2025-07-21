<!-- Setup and load i18n -->
<script lang="ts" module>
  import { init, register, getLocaleFromNavigator } from "svelte-i18n";

  // Register languages
  register("en", () => import("../../i18n/locales/en.json"));
  register("de", () => import("../../i18n/locales/de.json"));
  register("es", () => import("../../i18n/locales/es.json"));
  register("fr", () => import("../../i18n/locales/fr.json"));
  register("cs", () => import("../../i18n/locales/cs.json"));

  // Initialize i18n
  init({
    fallbackLocale: "en",
    initialLocale: getLocaleFromNavigator(),
  });
</script>

<script lang="ts">
  import { type Snippet } from "svelte";
  import { isLoading, locale as svelteLocale } from "svelte-i18n";

  import SkeletonList from "../skeleton/SkeletonList.svelte";

  type Props = {
    locale: string;
    children?: Snippet;
  };

  const { locale, children }: Props = $props();

  $effect(() => {
    svelteLocale.set(locale);
  });
</script>

{#if $isLoading}
  <SkeletonList />
{:else}
  {@render children?.()}
{/if}
