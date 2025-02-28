<script lang="ts">
  import SolarDevicesBoldDuotone from "~icons/solar/devices-bold-duotone";
  import SolarSettingsBoldDuotone from "~icons/solar/settings-bold-duotone";
  import SolarLayersMinimalisticBoldDuotone from "~icons/solar/layers-minimalistic-bold-duotone";

  import NavButton from "./NavButton.svelte";
</script>

<header class="header">
  <nav class="nav">
    <NavButton
      icon={SolarLayersMinimalisticBoldDuotone}
      href="/"
      title="Profiles"
    />
    <NavButton icon={SolarDevicesBoldDuotone} href="/devices" title="Devices" />
    <NavButton
      icon={SolarSettingsBoldDuotone}
      href="/settings"
      title="Settings"
    />
  </nav>
</header>

<style>
  .header {
    width: 100%;
    background-color: #29262e;
    padding: 0.5rem;
  }

  .nav {
    display: flex;
    gap: 0.5rem;
  }
</style>
