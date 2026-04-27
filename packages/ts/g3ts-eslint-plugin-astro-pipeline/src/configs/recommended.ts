import type { Linter } from "eslint";

export const recommendedRules = {
  "astro-pipeline/mdx-component-imports-from-approved-map": "error",
  "astro-pipeline/mdx-component-map-no-raw-ui-exports": "error",
  "astro-pipeline/mdx-component-wrapper-requires-zod-parse": "error",
  "astro-pipeline/mdx-imports-only-approved-components": "error",
  "astro-pipeline/no-authored-content-fs-read": "error",
  "astro-pipeline/no-authored-content-glob": "error",
  "astro-pipeline/no-authored-content-imports": "error",
  "astro-pipeline/no-content-data-modules-in-routes": "error",
  "astro-pipeline/no-direct-astro-content-in-routes": "error",
  "astro-pipeline/no-raw-mdx-images": "error",
  "astro-pipeline/no-runtime-mdx-eval": "error",
  "astro-pipeline/no-side-loader-imports": "error",
  "astro-pipeline/no-velite-imports": "error",
  "astro-pipeline/require-approved-content-adapter-in-routes": "error",
  "astro-pipeline/require-approved-json-ld-helper-in-routes": "error",
  "astro-pipeline/require-approved-metadata-helper-in-routes": "error"
} as const satisfies Linter.RulesRecord;

const recommended = {
  rules: recommendedRules
} as const satisfies Linter.Config;

export default recommended;
