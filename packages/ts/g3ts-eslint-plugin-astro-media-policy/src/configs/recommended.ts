import type { Linter } from "eslint";

export const recommendedRules = {
  "astro-media-policy/no-inline-image-alt": "error",
  "astro-media-policy/no-raw-public-image-paths": "error",
  "astro-media-policy/require-approved-media-helper": "error",
  "astro-media-policy/require-content-image-key": "error"
} as const satisfies Linter.RulesRecord;

const recommended = {
  rules: recommendedRules
} as const satisfies Linter.Config;

export default recommended;
