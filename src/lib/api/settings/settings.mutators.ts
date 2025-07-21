import { createMutation, getQueryClientContext } from "@tanstack/svelte-query";

import type { SettingsConfig } from "../types/settings";

import { settingsKeys } from "./settings.keys";
import { setSettings } from "./settings.requests";

export function createSetSettingsMutation() {
  const queryClient = getQueryClientContext();
  return createMutation({
    scope: {
      id: "settings",
    },
    mutationFn: ({ settings }: { settings: SettingsConfig }) =>
      setSettings(settings),
    onSuccess: (data) => {
      queryClient.setQueryData(settingsKeys.root, data);
    },
  });
}
