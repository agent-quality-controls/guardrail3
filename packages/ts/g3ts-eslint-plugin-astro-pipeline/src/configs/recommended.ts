import type { Linter } from "eslint";

export const recommendedRules = {
  "astro-pipeline/no-authored-content-fs-read": "error",
  "astro-pipeline/no-authored-content-glob": "error",
  "astro-pipeline/no-authored-content-imports": "error",
  "astro-pipeline/no-content-data-modules-in-routes": "error",
  "astro-pipeline/no-direct-astro-content-in-routes": "error",
  "astro-pipeline/no-runtime-mdx-eval": "error",
  "astro-pipeline/no-side-loader-imports": "error",
  "astro-pipeline/no-velite-imports": "error",
  "astro-pipeline/require-approved-content-adapter-in-routes": "error"
} as const satisfies Linter.RulesRecord;

const recommended = {
  rules: recommendedRules
} as const satisfies Linter.Config;

export default recommended;
