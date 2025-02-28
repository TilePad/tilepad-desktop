<script lang="ts">
  import SolarDevicesBoldDuotone from "~icons/solar/devices-bold-duotone";
  import SolarSettingsBoldDuotone from "~icons/solar/settings-bold-duotone";
  import SolarLayersMinimalisticBoldDuotone from "~icons/solar/layers-minimalistic-bold-duotone";
  import SolarPlugCircleBoldDuotone from "~icons/solar/plug-circle-bold-duotone";
  import NavButton from "./NavButton.svelte";
</script>

<header class="header">
  <img class="logo" src="/tilepad-logo.svg" alt="Logo" />
  <nav class="nav">
    <NavButton
      icon={SolarLayersMinimalisticBoldDuotone}
      href="/"
      title="Profiles"
    />
    <NavButton icon={SolarDevicesBoldDuotone} href="/devices" title="Devices" />
    <NavButton
      icon={SolarPlugCircleBoldDuotone}
      href="/plugins"
      title="Plugins"
    />
    <NavButton
      icon={SolarSettingsBoldDuotone}
      href="/settings"
      title="Settings"
    />
  </nav>
</header>

<style>
  .header {
    display: flex;
    gap: 1rem;
    width: 100%;
    background-color: #29262e;
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
