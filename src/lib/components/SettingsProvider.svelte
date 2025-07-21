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

  type Props = {
    settings: SettingsConfig;
    children?: Snippet;
  };

  const { children, settings }: Props = $props();

  setContext(settingsContextKey, {
    settings() {
      return settings!;
    },
  });
</script>

{@render children?.()}
