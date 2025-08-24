<script lang="ts">
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import tilepadLogo from "$lib/assets/tilepad-logo.svg";
  import SolarDevicesBoldDuotone from "~icons/solar/devices-bold-duotone";
  import SolarSettingsBoldDuotone from "~icons/solar/settings-bold-duotone";
  import SolarPlugCircleBoldDuotone from "~icons/solar/plug-circle-bold-duotone";
  import SolarGalleryWideBoldDuotone from "~icons/solar/gallery-wide-bold-duotone";
  import SolarLayersMinimalisticBoldDuotone from "~icons/solar/layers-minimalistic-bold-duotone";

  import NavButton from "./NavButton.svelte";

  const i18n = i18nContext.get();
</script>

<header class="header">
  <img class="logo" src={tilepadLogo} alt="Logo" />
  <nav class="nav">
    <NavButton
      icon={SolarLayersMinimalisticBoldDuotone}
      href="/"
      title={i18n.f("profiles")}
    />
    <NavButton
      icon={SolarDevicesBoldDuotone}
      href="/devices"
      title={i18n.f("devices")}
    />
    <NavButton
      icon={SolarPlugCircleBoldDuotone}
      href="/plugins"
      title={i18n.f("plugins")}
    />
    <NavButton
      icon={SolarGalleryWideBoldDuotone}
      href="/icons"
      title={i18n.f("icon_packs")}
    />
    <NavButton
      icon={SolarSettingsBoldDuotone}
      href="/settings"
      title={i18n.f("settings")}
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
