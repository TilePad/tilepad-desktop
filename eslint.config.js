// @ts-check

import globals from "globals";
import eslint from "@eslint/js";
import tseslint from "typescript-eslint";
import * as svelteParser from "svelte-eslint-parser";
import eslintPluginSvelte from "eslint-plugin-svelte";
import perfectionist from "eslint-plugin-perfectionist";
import unusedImports from "eslint-plugin-unused-imports";
import tanstackQuery from "@tanstack/eslint-plugin-query";
import * as typescriptParser from "@typescript-eslint/parser";

export default tseslint.config(
  { ignores: ["src-tauri/src/script/**/*", "script/**/*"] },
  eslint.configs.recommended,
  tseslint.configs.recommended,
  ...eslintPluginSvelte.configs["flat/recommended"],
  ...eslintPluginSvelte.configs["flat/prettier"],
  ...tanstackQuery.configs["flat/recommended"],
  {
    files: ["**/*.svelte", "**/*.svelte.ts"],
    languageOptions: {
      parser: svelteParser,
      parserOptions: {
        parser: {
          // Specify a parser for each lang.
          ts: typescriptParser,
          typescript: typescriptParser,
        },
        project: "./tsconfig.json",
        extraFileExtensions: [".svelte"],
      },
    },
  },
  {
    languageOptions: {
      globals: {
        ...globals.browser,
      },
    },
  },
  {
    plugins: {
      perfectionist,
      "unused-imports": unusedImports,
    },
    rules: {
      "no-unused-vars": "off",
      "no-undef": "off",
      "unused-imports/no-unused-imports": "warn",
      "unused-imports/no-unused-vars": [
        "off",
        {
          vars: "all",
          varsIgnorePattern: "^_",
          args: "after-used",
          argsIgnorePattern: "^_",
        },
      ],
      "@typescript-eslint/no-unused-vars": [
        "warn",
        {
          args: "none",
          argsIgnorePattern: "^_",
          varsIgnorePattern: "^_",
          caughtErrorsIgnorePattern: "^_",
        },
      ],

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
