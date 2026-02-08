<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import UpdaterPrompt from "./UpdaterPrompt.svelte";

  const { Story } = defineMeta({
    title: "Update/UpdaterPrompt",
    component: UpdaterPrompt,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story
  name="Ready"
  args={{
    update: {
      version: "0.10.0",
      download: () => Promise.resolve(),
      install: () => Promise.resolve(),
    },
    state: "ready",
    onUpdate: () => {},
    onClose: () => {},
  }}
/>

<Story
  name="Downloading"
  args={{
    update: {
      version: "0.10.0",
      download: () => Promise.resolve(),
      install: () => Promise.resolve(),
    },
    state: "downloading",
    onUpdate: () => {},
    onClose: () => {},
  }}
/>

<Story
  name="Installable"
  args={{
    update: {
      version: "0.10.0",
      download: () => Promise.resolve(),
      install: () => Promise.resolve(),
    },
    state: "installable",
    onUpdate: () => {},
    onClose: () => {},
  }}
/>

<Story
  name="Installing"
  args={{
    update: {
      version: "0.10.0",
      download: () => Promise.resolve(),
      install: () => Promise.resolve(),
    },
    state: "installing",
    onUpdate: () => {},
    onClose: () => {},
  }}
/>
