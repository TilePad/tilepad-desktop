<script lang="ts">
  import type { SettingsConfig } from "$lib/api/types/settings";

  import { t } from "svelte-i18n";
  import { getVersion } from "@tauri-apps/api/app";
  import { watch, resource, useDebounce } from "runed";
  import { createSetSettingsMutation } from "$lib/api/settings";
  import TextInput from "$lib/components/input/TextInput.svelte";
  import CreatorSection from "$lib/components/CreatorSection.svelte";
  import NumberInput from "$lib/components/input/NumberInput.svelte";
  import EnabledSwitch from "$lib/components/input/EnabledSwitch.svelte";
  import LanguageSelector from "$lib/components/i18n/LanguageSelector.svelte";
  import { getSettingsContext } from "$lib/components/SettingsProvider.svelte";
  import LicensesDialog from "$lib/components/liceneses/LicensesDialog.svelte";
  import SolarDocumentAddBoldDuotone from "~icons/solar/document-add-bold-duotone";

  const settingsContext = getSettingsContext();
  const currentSettings = $derived.by(settingsContext.settings);
  const setSettings = createSetSettingsMutation();

  const version = resource(
    () => getVersion(),
    (promise) => promise,
  );

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

  const onChangeServerPort = (port: number) => {
    updateSettings({ ...settings, port });
  };

  const onChangeLanguage = (language: string) => {
    updateSettings({ ...settings, language });
  };

  const onChangeDeveloperMode = (developer_mode: boolean) => {
    updateSettings({ ...settings, developer_mode });
  };

  const onChangeAutoStart = (start_automatically: boolean) => {
    updateSettings({ ...settings, start_automatically });
  };

  const onChangeStartMinimized = (start_minimized: boolean) => {
    updateSettings({ ...settings, start_minimized });
  };

  const onChangeMinimizeTray = (minimize_tray: boolean) => {
    updateSettings({ ...settings, minimize_tray });
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

    <div class="actions"></div>
  </div>

  <div class="settings">
    <div class="grid">
      <div class="card">
        <div class="tile-item">
          <label class="tile-label" for="language">{$t("language")}</label>
          <LanguageSelector
            value={settings.language}
            onChangeValue={(value) => onChangeLanguage(value)}
          />
          <p class="tile-description">{$t("language_description")}</p>
        </div>
      </div>

      <div class="card">
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
      </div>

      <div class="card">
        <div class="dev-row">
          <div class="tile-item">
            <label class="tile-label" for="startAutomatically"
              >{$t("auto_start")}</label
            >
            <p class="tile-description">
              {$t("auto_start_description")}
            </p>
          </div>

          <EnabledSwitch
            id="startAutomatically"
            checked={settings.start_automatically}
            onCheckedChange={(value) => onChangeAutoStart(value)}
          />
        </div>
      </div>

      <div class="card">
        <div class="dev-row">
          <div class="tile-item">
            <label class="tile-label" for="startMinimized"
              >{$t("start_minimized")}</label
            >
            <p class="tile-description">
              {$t("start_minimized_description")}
            </p>
          </div>

          <EnabledSwitch
            id="startMinimized"
            checked={settings.start_minimized}
            onCheckedChange={(value) => onChangeStartMinimized(value)}
          />
        </div>
      </div>

      <div class="card">
        <div class="dev-row">
          <div class="tile-item">
            <label class="tile-label" for="startAutomatically"
              >{$t("minimize_tray")}</label
            >
            <p class="tile-description">
              {$t("minimize_tray_description")}
            </p>
          </div>

          <EnabledSwitch
            id="startAutomatically"
            checked={settings.minimize_tray}
            onCheckedChange={(value) => onChangeMinimizeTray(value)}
          />
        </div>
      </div>

      <div class="card">
        <div class="dev-row">
          <div class="tile-item">
            <label class="tile-label" for="developmentMode"
              >{$t("development_mode")}</label
            >
            <p class="tile-description">
              {$t("development_mode_description")}
            </p>
          </div>

          <EnabledSwitch
            id="developmentMode"
            checked={settings.developer_mode}
            onCheckedChange={(value) => onChangeDeveloperMode(value)}
          />
        </div>
      </div>
    </div>

    <div class="card">
      <div class="tile-item">
        <label class="tile-label" for="serverPort">{$t("server_port")}</label>
        <NumberInput
          style="width: 100%"
          id="serverPort"
          value={settings.port}
          onchange={(event) =>
            onChangeServerPort(event.currentTarget.valueAsNumber)}
        />

        <p class="tile-description">
          {$t("server_port_description")}
        </p>
      </div>
    </div>

    <div class="card">
      <div class="dev-row">
        <div class="tile-item">
          <label class="tile-label" for="third_party_licenses">
            {$t("third_party_licenses")}
          </label>
          <p class="tile-description">
            {$t("third_party_licenses_description")}
          </p>
        </div>
        <LicensesDialog
          buttonLabel={{
            text: $t("third_party_licenses"),
            icon: SolarDocumentAddBoldDuotone,
          }}
        />
      </div>
    </div>

    <CreatorSection version={version.current} />
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
    gap: 1rem;
    padding: 1rem;
    flex: auto;
    overflow: auto;
  }

  .dev-row {
    display: flex;
    gap: 1rem;
    align-items: center;
    width: 100%;
    justify-content: space-between;
    padding-right: 1rem;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
  }

  .card {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;

    padding: 0.5rem;
    border-radius: 0.5rem;
    background-color: #2f2c36;
  }
</style>
