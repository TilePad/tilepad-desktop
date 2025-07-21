import { createQuery } from "@tanstack/svelte-query";

import { settingsKeys } from "./settings.keys";
import { getSettings } from "./settings.requests";

export function createSettingsQuery() {
  return createQuery({
    queryKey: settingsKeys.root,
    queryFn: getSettings,
  });
}
