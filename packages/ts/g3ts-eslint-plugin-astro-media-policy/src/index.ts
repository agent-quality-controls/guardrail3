import recommended from "./configs/recommended.js";
import noInlineImageAlt from "./rules/no-inline-image-alt.js";
import noRawPublicImagePaths from "./rules/no-raw-public-image-paths.js";
import requireApprovedMediaHelper from "./rules/require-approved-media-helper.js";
import requireContentImageKey from "./rules/require-content-image-key.js";

const rules = {
  "no-inline-image-alt": noInlineImageAlt,
  "no-raw-public-image-paths": noRawPublicImagePaths,
  "require-approved-media-helper": requireApprovedMediaHelper,
  "require-content-image-key": requireContentImageKey
} as const;

const plugin = {
  meta: {
    name: "g3ts-eslint-plugin-astro-media-policy"
  },
  configs: {
    recommended
  },
  rules
};

export { recommended, rules };
export default plugin;
