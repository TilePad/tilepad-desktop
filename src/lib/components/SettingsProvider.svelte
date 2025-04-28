<script lang="ts" module>
  import { getContext, setContext, type Snippet } from "svelte";

  type SettingsContext = {
    settings: () => SettingsConfig;
  };

  const settingsContextKey = Symbol("SETTINGS_CONTEXT");

  export function getSettingsContext(): SettingsContext {
    return getContext(settingsContextKey);
  }
</script>

<script lang="ts">
  import type { SettingsConfig } from "$lib/api/types/settings";

  import { t } from "svelte-i18n";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { createSettingsQuery } from "$lib/api/settings";

  import Aside from "./Aside.svelte";
  import SkeletonList from "./skeleton/SkeletonList.svelte";

  type Props = {
    children?: Snippet;
  };

  const { children }: Props = $props();

  const settingsQuery = createSettingsQuery();
  const settings = $derived($settingsQuery.data);

  setContext(settingsContextKey, {
    settings() {
      return settings!;
    },
  });
</script>

{#if $settingsQuery.isLoading}
  <SkeletonList style="margin: 1rem;" />
{:else if $settingsQuery.isError}
  <!-- Error creating current profile -->
  <Aside severity="error" style="margin: 1rem;">
    {$t("settings_error", {
      values: { error: getErrorMessage($settingsQuery.error) },
    })}
  </Aside>
{:else if $settingsQuery.isSuccess && settings}
  {@render children?.()}
{/if}
