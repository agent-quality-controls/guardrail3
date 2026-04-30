import type { Linter } from "eslint";

export const recommendedRules = {
  "astro-i18n-policy/no-unlocalized-internal-hrefs": "error"
} as const satisfies Linter.RulesRecord;

const recommended = {
  rules: recommendedRules
} as const satisfies Linter.Config;

export default recommended;
