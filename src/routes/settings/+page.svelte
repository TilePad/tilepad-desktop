<script lang="ts">
  import type { SettingsConfig } from "$lib/api/types/settings";

  import { watch, useDebounce } from "runed";
  import { createSetSettingsMutation } from "$lib/api/settings";
  import TextInput from "$lib/components/input/TextInput.svelte";
  import LanguageSelector from "$lib/components/i18n/LanguageSelector.svelte";
  import { getSettingsContext } from "$lib/components/SettingsProvider.svelte";

  const settingsContext = getSettingsContext();
  const currentSettings = $derived.by(settingsContext.settings);
  const setSettings = createSetSettingsMutation();

  // (Initial value of settings used for initial state)
  const defaultSettings = settingsContext.settings();

  let settings = $state(defaultSettings);
  let lastSettingsUpdate: SettingsConfig = $state(defaultSettings);

  const updateSettingsDebounce = useDebounce((settings: SettingsConfig) => {
    lastSettingsUpdate = settings;
    $setSettings.mutate({
      settings,
    });
  }, 150);

  const updateSettings = (newSettings: SettingsConfig) => {
    settings = newSettings;
    updateSettingsDebounce(newSettings);
  };

  const onChangeDeviceName = (name: string) => {
    updateSettings({ ...settings, device_name: name });
  };

  const onChangeLanguage = (language: string) => {
    updateSettings({ ...settings, language });
  };

  // Update local settings state with remote
  watch(
    () => ({ currentSettings }),
    ({ currentSettings }) => {
      if (
        JSON.stringify(lastSettingsUpdate) === JSON.stringify(currentSettings)
      )
        return;
      settings = currentSettings;
    },
  );
</script>

<div class="settings">
  <div class="tile-item">
    <label class="tile-label" for="language">Language</label>
    <LanguageSelector
      value={settings.language}
      onChangeValue={(value) => onChangeLanguage(value)}
    />
    <p class="tile-description">Language for the user interface</p>
  </div>

  <div class="tile-item">
    <label class="tile-label" for="deviceName">Device Name</label>
    <TextInput
      style="width: 100%"
      id="deviceName"
      value={settings.device_name}
      onchange={(event) => onChangeDeviceName(event.currentTarget.value)}
    />
    <p class="tile-description">
      This device name is visible to other devices attempting to connect to this
      device
    </p>
  </div>
</div>

<style>
  .tile-item {
    padding: 8px;
  }

  .tile-label {
    display: block;
    font-weight: 600;
    color: #ddd;
    font-weight: bold;
    margin-bottom: 2px;
  }

  .tile-description {
    display: block;
    font-size: 12px;
    color: #aaa;
    margin-top: 4px;
  }
</style>
