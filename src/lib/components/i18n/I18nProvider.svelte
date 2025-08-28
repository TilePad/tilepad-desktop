<script lang="ts">
  import { type Snippet } from "svelte";
  import { createI18n, i18nContext } from "$lib/i18n/i18n.svelte";

  import SkeletonList from "../skeleton/SkeletonList.svelte";

  type Props = {
    children?: Snippet;
  };

  const { children }: Props = $props();

  const i18n = createI18n();
  i18nContext.set(i18n);
</script>

{#if i18n.loading}
  <SkeletonList />
{:else}
  {@render children?.()}
{/if}
