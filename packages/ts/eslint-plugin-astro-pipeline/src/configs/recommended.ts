import i18next from "eslint-plugin-i18next";

import type { Linter } from "eslint";

export const recommendedRules = {
  "astro-pipeline/no-authored-content-fs-read": "error",
  "astro-pipeline/no-authored-content-glob": "error",
  "astro-pipeline/no-authored-content-imports": "error",
  "astro-pipeline/no-content-data-modules-in-routes": "error",
  "astro-pipeline/no-direct-astro-content-in-routes": "error",
  "astro-pipeline/no-runtime-mdx-eval": "error",
  "astro-pipeline/no-side-loader-imports": "error",
  "astro-pipeline/no-velite-imports": "error"
} as const satisfies Linter.RulesRecord;

export const noInlinePublicContentOptions = {
  framework: "react",
  mode: "all",
  message:
    "Inline public copy must live in Astro content entries. Move this text into the content collection, validate it through the collection schema, and pass the typed value into source.",
  "should-validate-template": true,
  words: {
    exclude: ["[0-9!-/:-@[-`{-~]+", "[A-Z_-]+"]
  },
  "jsx-components": {
    include: [],
    exclude: []
  },
  "jsx-attributes": {
    include: [],
    exclude: [
      "as",
      "class",
      "className",
      "color",
      "data-.+",
      "height",
      "href",
      "id",
      "intent",
      "key",
      "name",
      "rel",
      "role",
      "size",
      "slot",
      "src",
      "style",
      "styleName",
      "target",
      "tone",
      "type",
      "variant",
      "width",
      "aria-hidden"
    ]
  },
  callees: {
    include: [],
    exclude: ["require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL"]
  },
  "object-properties": {
    include: [],
    exclude: ["[A-Z_-]+"]
  },
  "class-properties": {
    include: [],
    exclude: ["displayName"]
  }
} as const;

const recommended = {
  rules: recommendedRules
} as const satisfies Linter.Config;

export const strictContent = {
  plugins: {
    i18next
  },
  rules: {
    "i18next/no-literal-string": ["error", noInlinePublicContentOptions]
  }
} as const satisfies Linter.Config;

export default recommended;
