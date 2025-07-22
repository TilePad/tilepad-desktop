import type { Preview } from "@storybook/sveltekit";

import RootLayout from "../src/lib/layouts/RootLayout.svelte";
import I18nProvider from "../src/lib/components/i18n/I18nProvider.svelte";

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
  },
  decorators: [
    () => RootLayout,
    () => ({
      Component: I18nProvider,
      props: {
        locale: "en",
      },
    }),
  ],
};

export default preview;
