// [KEYS] ------------------------------------------------------

import { invoke } from "@tauri-apps/api/core";
import { createQuery, createMutation } from "@tanstack/svelte-query";

import type { SettingsConfig } from "./types/settings";

import { queryClient } from "./client";

const settingsKeys = {
  root: ["settings"],
};

// [REQUESTS] ------------------------------------------------------

async function getSettings(): Promise<SettingsConfig> {
  return invoke<SettingsConfig>("settings_get_settings");
}

async function setSettings(settings: SettingsConfig) {
  return invoke<SettingsConfig>("settings_set_settings", { settings });
}

// [QUERIES] ------------------------------------------------------

export function createSettingsQuery() {
  return createQuery({
    queryKey: settingsKeys.root,
    queryFn: getSettings,
  });
}

// [MUTATORS] ------------------------------------------------------

export function createSetSettingsMutation() {
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
