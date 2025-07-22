<script lang="ts">
  import { t } from "svelte-i18n";
  import tilepadLogo from "$lib/assets/tilepad-logo.svg";
  import SolarDevicesBoldDuotone from "~icons/solar/devices-bold-duotone";
  import SolarSettingsBoldDuotone from "~icons/solar/settings-bold-duotone";
  import SolarPlugCircleBoldDuotone from "~icons/solar/plug-circle-bold-duotone";
  import SolarGalleryWideBoldDuotone from "~icons/solar/gallery-wide-bold-duotone";
  import SolarLayersMinimalisticBoldDuotone from "~icons/solar/layers-minimalistic-bold-duotone";

  import NavButton from "./NavButton.svelte";
</script>

<header class="header">
  <img class="logo" src={tilepadLogo} alt="Logo" />
  <nav class="nav">
    <NavButton
      icon={SolarLayersMinimalisticBoldDuotone}
      href="/"
      title={$t("profiles")}
    />
    <NavButton
      icon={SolarDevicesBoldDuotone}
      href="/devices"
      title={$t("devices")}
    />
    <NavButton
      icon={SolarPlugCircleBoldDuotone}
      href="/plugins"
      title={$t("plugins")}
    />
    <NavButton
      icon={SolarGalleryWideBoldDuotone}
      href="/icons"
      title={$t("icon_packs")}
    />
    <NavButton
      icon={SolarSettingsBoldDuotone}
      href="/settings"
      title={$t("settings")}
    />
  </nav>
</header>

<style>
  .header {
    display: flex;
    gap: 1rem;
    width: 100%;
    background-color: #322e38;
    padding: 0.5rem;
    align-items: center;
  }

  .logo {
    max-height: 2rem;
  }

  .nav {
    display: flex;
    gap: 0.5rem;
  }
</style>
