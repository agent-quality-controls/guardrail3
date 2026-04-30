import type { Linter } from "eslint";

export const recommendedRules = {
  "style-policy/no-denied-class-tokens": "error"
} as const satisfies Linter.RulesRecord;

const recommended = {
  rules: recommendedRules
} as const satisfies Linter.Config;

export default recommended;
