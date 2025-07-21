import type { Preview } from "@storybook/sveltekit";
import RootLayout from "../src/lib/layouts/RootLayout.svelte";

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
  },
  decorators: [() => RootLayout],
};

export default preview;
