import { invoke } from "@tauri-apps/api/core";

import type { SettingsConfig } from "../types/settings";

export function getSettings(): Promise<SettingsConfig> {
  return invoke<SettingsConfig>("settings_get_settings");
}

export function setSettings(settings: SettingsConfig) {
  return invoke<SettingsConfig>("settings_set_settings", { settings });
}
