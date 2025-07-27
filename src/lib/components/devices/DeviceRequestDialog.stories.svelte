<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import DeviceRequestDialog from "./DeviceRequestDialog.svelte";

  const { Story } = defineMeta({
    title: "Devices/DeviceRequestDialog",
    component: DeviceRequestDialog,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story
  name="Default"
  args={{
    deviceName: "Example Device",
    address: "192.168.0.1",
    clientPublicKey: new Array(32).fill(0),

    onApprove: () => {},
    onDecline: () => {},
  }}
/>
