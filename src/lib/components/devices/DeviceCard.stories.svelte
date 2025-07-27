<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import DeviceCard from "./DeviceCard.svelte";

  const { Story } = defineMeta({
    title: "Devices/DeviceCard",
    component: DeviceCard,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story
  name="Default"
  args={{
    id: "00000000-0000-0000-0000-000000000000",
    name: "Example Device",
    publicKey: new Array(32).fill(0),
    profileId: "00000000-0000-0000-0000-000000000000",
    folderId: "00000000-0000-0000-0000-000000000000",
    connected: false,
    profiles: [
      {
        value: "00000000-0000-0000-0000-000000000000",
        name: "Default Profile",
      },
    ],
    folders: [
      {
        value: "00000000-0000-0000-0000-000000000000",
        name: "Default Folder",
      },
    ],
    onRevoke: () => {},
    onChangeProfile: () => {},
    onChangeFolder: () => {},
  }}
/>

<Story
  name="Connected"
  args={{
    id: "00000000-0000-0000-0000-000000000000",
    name: "Example Device",
    publicKey: new Array(32).fill(0),
    profileId: "00000000-0000-0000-0000-000000000000",
    folderId: "00000000-0000-0000-0000-000000000000",
    connected: true,
    profiles: [
      {
        value: "00000000-0000-0000-0000-000000000000",
        name: "Default Profile",
      },
    ],
    folders: [
      {
        value: "00000000-0000-0000-0000-000000000000",
        name: "Default Folder",
      },
    ],
    onRevoke: () => {},
    onChangeProfile: () => {},
    onChangeFolder: () => {},
  }}
/>

<Story
  name="No Folder"
  args={{
    id: "00000000-0000-0000-0000-000000000000",
    name: "Example Device",
    publicKey: new Array(32).fill(0),
    profileId: "00000000-0000-0000-0000-000000000000",
    folderId: null,
    connected: true,
    profiles: [
      {
        value: "00000000-0000-0000-0000-000000000000",
        name: "Default Profile",
      },
    ],
    folders: [],
    onRevoke: () => {},
    onChangeProfile: () => {},
    onChangeFolder: () => {},
  }}
/>
