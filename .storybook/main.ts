import type { StorybookConfig } from "@storybook/sveltekit";

const config: StorybookConfig = {
  stories: ["../src/**/*.mdx", "../src/**/*.stories.@(js|ts|svelte)"],
  addons: ["@storybook/addon-svelte-csf"],
  framework: "@storybook/sveltekit",
  // framework: {
  //   name: getAbsolutePath("@storybook/sveltekit"),
  //   options: {},
  // },
};
export default config;
