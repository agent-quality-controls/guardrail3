import recommended from "./configs/recommended.js";
import noUnlocalizedInternalHrefs from "./rules/no-unlocalized-internal-hrefs.js";

const rules = {
  "no-unlocalized-internal-hrefs": noUnlocalizedInternalHrefs
} as const;

const plugin = {
  meta: {
    name: "g3ts-eslint-plugin-astro-i18n-policy"
  },
  configs: {
    recommended
  },
  rules
};

export { recommended, rules };
export default plugin;
