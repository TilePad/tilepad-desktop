import js from "@eslint/js";
import globals from "globals";
import ts from "typescript-eslint";
import { fileURLToPath } from "node:url";
import svelte from "eslint-plugin-svelte";
import prettier from "eslint-config-prettier";
import storybook from "eslint-plugin-storybook";
import { includeIgnoreFile } from "@eslint/compat";
import perfectionist from "eslint-plugin-perfectionist";
import unusedImports from "eslint-plugin-unused-imports";
import tanstackQuery from "@tanstack/eslint-plugin-query";

import svelteConfig from "./svelte.config.js";

const gitignorePath = fileURLToPath(new URL("./.gitignore", import.meta.url));

export default ts.config(
  // Ignored files
  includeIgnoreFile(gitignorePath),
  { ignores: ["src-tauri/src/script/**/*", "script/**/*"] },
  // JS
  js.configs.recommended,
  // TS
  ...ts.configs.recommended,
  // Svelte
  ...svelte.configs.recommended,
  // Prettier
  prettier,
  ...svelte.configs.prettier,
  // Tanstack
  tanstackQuery.configs["flat/recommended"],
  // Globals and undef
  {
    languageOptions: {
      globals: { ...globals.browser, ...globals.node },
    },
    rules: {
      // typescript-eslint strongly recommend that you do not use the no-undef lint rule on TypeScript projects.
      // see: https://typescript-eslint.io/troubleshooting/faqs/eslint/#i-get-errors-from-the-no-undef-rule-about-global-variables-not-being-defined-even-though-there-are-no-typescript-errors
      "no-undef": "off",
    },
  },
  // Svelte files
  {
    files: ["**/*.svelte", "**/*.svelte.ts", "**/*.svelte.js"],
    languageOptions: {
      parserOptions: {
        // projectService: true,
        extraFileExtensions: [".svelte"],
        parser: ts.parser,
        svelteConfig,
      },
    },
  },
  // Storybook
  storybook.configs["flat/recommended"],
  // Unused imports
  {
    plugins: {
      "unused-imports": unusedImports,
    },
    rules: {
      "no-unused-vars": "off",
      "@typescript-eslint/no-unused-vars": "off",
      "unused-imports/no-unused-imports": "warn",
      "unused-imports/no-unused-vars": [
        "warn",
        {
          vars: "all",
          varsIgnorePattern: "^_",
          args: "after-used",
          argsIgnorePattern: "^_",
        },
      ],
    },
  },
  // Perfectionist
  {
    plugins: {
      perfectionist,
    },
    rules: {
      "perfectionist/sort-named-imports": [
        "warn",
        {
          order: "asc",
          type: "line-length",
        },
      ],
      "perfectionist/sort-named-exports": [
        "warn",
        {
          order: "asc",
          type: "line-length",
        },
      ],
      "perfectionist/sort-exports": [
        "warn",
        {
          order: "asc",
          type: "line-length",
        },
      ],
      "perfectionist/sort-imports": [
        "warn",
        {
          order: "asc",
          type: "line-length",
          newlinesBetween: "always",
          internalPattern: ["^~/.*"],
        },
      ],
    },
  },
);
