<script lang="ts">
  import type { Snippet } from "svelte";
  import type { SettingsConfig } from "$lib/api/types/settings";

  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { createSettingsQuery } from "$lib/api/settings";

  import Aside from "./Aside.svelte";
  import SkeletonList from "./skeleton/SkeletonList.svelte";

  type Props = {
    children?: Snippet<[{ settings: SettingsConfig }]>;
  };

  const { children }: Props = $props();

  const i18n = i18nContext.get();

  const settingsQuery = createSettingsQuery();
</script>

{#if settingsQuery.isLoading}
  <SkeletonList style="margin: 1rem;" />
{:else if settingsQuery.isError}
  <!-- Error creating current profile -->
  <Aside severity="error" style="margin: 1rem;">
    {i18n.f("settings_error", {
      values: { error: getErrorMessage(settingsQuery.error) },
    })}
  </Aside>
{:else if settingsQuery.isSuccess}
  {@render children?.({ settings: settingsQuery.data })}
{/if}
