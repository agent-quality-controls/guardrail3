import recommended from "./configs/recommended.js";
import noInlineImageAlt from "./rules/no-inline-image-alt.js";
import noUnlocalizedInternalHrefs from "./rules/no-unlocalized-internal-hrefs.js";
import requireContentImageKey from "./rules/require-content-image-key.js";

const rules = {
  "no-inline-image-alt": noInlineImageAlt,
  "no-unlocalized-internal-hrefs": noUnlocalizedInternalHrefs,
  "require-content-image-key": requireContentImageKey
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
