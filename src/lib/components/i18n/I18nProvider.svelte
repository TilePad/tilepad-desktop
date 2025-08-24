<script lang="ts">
  import { watch } from "runed";
  import { type Snippet } from "svelte";
  import { i18nContext } from "$lib/i18n/i18n.svelte";

  import SkeletonList from "../skeleton/SkeletonList.svelte";

  type Props = {
    locale: string;
    children?: Snippet;
  };

  const { locale, children }: Props = $props();

  const context = i18nContext.get();

  watch(
    () => ({ context, locale }),
    ({ context, locale }) => {
      context.locale = locale;
    },
  );
</script>

{#if context.loading}
  <SkeletonList />
{:else}
  {@render children?.()}
{/if}
