import type { ESLint, Linter } from "eslint";

import plugin from "../plugin.js";
import type { StylePolicyOptions } from "../utils/options.js";

export function recommended(options: StylePolicyOptions): Linter.Config {
  return {
    plugins: {
      "style-policy": plugin as unknown as ESLint.Plugin
    },
    rules: {
      "style-policy/no-denied-class-tokens": ["error", options]
    }
  };
}

export default recommended;
