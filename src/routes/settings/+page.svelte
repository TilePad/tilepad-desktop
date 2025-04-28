<script lang="ts">
  import type { SettingsConfig } from "$lib/api/types/settings";

  import { t } from "svelte-i18n";
  import { watch, useDebounce } from "runed";
  import { createSetSettingsMutation } from "$lib/api/settings";
  import TextInput from "$lib/components/input/TextInput.svelte";
  import CreatorSection from "$lib/components/CreatorSection.svelte";
  import LanguageSelector from "$lib/components/i18n/LanguageSelector.svelte";
  import { getSettingsContext } from "$lib/components/SettingsProvider.svelte";
  import LicensesDialog from "$lib/components/liceneses/LicensesDialog.svelte";
  import SolarDocumentAddBoldDuotone from "~icons/solar/document-add-bold-duotone";
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

<div class="layout">
  <div class="header">
    <h2>{$t("settings")}</h2>

    <div class="actions">
      <LicensesDialog
        buttonLabel={{
          text: $t("third_party_licenses"),
          icon: SolarDocumentAddBoldDuotone,
        }}
      />
    </div>
  </div>

  <div class="settings">
    <div class="tile-item">
      <label class="tile-label" for="language">{$t("language")}</label>
      <LanguageSelector
        value={settings.language}
        onChangeValue={(value) => onChangeLanguage(value)}
      />
      <p class="tile-description">{$t("language_description")}</p>
    </div>

    <div class="tile-item">
      <label class="tile-label" for="deviceName">{$t("device_name")}</label>
      <TextInput
        style="width: 100%"
        id="deviceName"
        value={settings.device_name}
        onchange={(event) => onChangeDeviceName(event.currentTarget.value)}
      />
      <p class="tile-description">
        {$t("device_name_description")}
      </p>
    </div>

    <CreatorSection />
  </div>
</div>

<style>
  .layout {
    height: 100%;
    overflow: hidden;

    display: flex;
    flex-flow: column;
  }

  .header {
    display: flex;
    flex-flow: row;
    flex-shrink: 0;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem;
    border-bottom: 1px solid #333;
    background-color: #29262e;
  }

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

  .settings {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
    padding: 1rem;
  }
</style>
