<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import DeviceConnectQR from "./DeviceConnectQR.svelte";

  const { Story } = defineMeta({
    title: "Devices/DeviceConnectQR",
    component: DeviceConnectQR,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story
  name="Default"
  args={{
    connectInfo: {
      interfaces: new Array(5).fill({ addr: "127.0.0.1", name: "Loopback" }),
      port: 3333,
    },
  }}
/>

<Story
  name="No Interfaces"
  args={{
    connectInfo: {
      interfaces: [],
      port: 3333,
    },
  }}
/>
