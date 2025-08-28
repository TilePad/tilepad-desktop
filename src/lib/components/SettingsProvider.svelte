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
  import { watch } from "runed";
  import { i18nContext } from "$lib/i18n/i18n.svelte";

  type Props = {
    settings: SettingsConfig;
    children?: Snippet;
  };

  const { children, settings }: Props = $props();

  const i18n = i18nContext.get();

  watch(
    () => ({ i18n, settings }),
    ({ i18n, settings }) => {
      i18n.locale = settings.language;
    },
  );

  setContext(settingsContextKey, {
    settings() {
      return settings!;
    },
  });
</script>

{@render children?.()}
