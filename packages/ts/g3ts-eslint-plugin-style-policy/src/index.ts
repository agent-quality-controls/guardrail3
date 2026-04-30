import recommended from "./configs/recommended.js";
import noDeniedClassTokens from "./rules/no-denied-class-tokens.js";

const rules = {
  "no-denied-class-tokens": noDeniedClassTokens
} as const;

const plugin = {
  meta: {
    name: "g3ts-eslint-plugin-style-policy"
  },
  configs: {
    recommended
  },
  rules
};

export { recommended, rules };
export default plugin;
