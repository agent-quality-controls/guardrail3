import noDeniedClassTokens from "./rules/no-denied-class-tokens.js";

const rules = {
  "no-denied-class-tokens": noDeniedClassTokens
} as const;

const plugin = {
  meta: {
    name: "g3ts-eslint-plugin-style-policy"
  },
  rules
};

export { rules };
export default plugin;
