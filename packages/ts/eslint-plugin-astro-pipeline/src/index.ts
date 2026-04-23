import recommended from "./configs/recommended.js";
import noAuthoredContentFsRead from "./rules/no-authored-content-fs-read.js";
import noAuthoredContentGlob from "./rules/no-authored-content-glob.js";
import noDirectAstroContentInRoutes from "./rules/no-direct-astro-content-in-routes.js";
import noRuntimeMdxEval from "./rules/no-runtime-mdx-eval.js";
import noSideLoaderImports from "./rules/no-side-loader-imports.js";

const rules = {
  "no-authored-content-fs-read": noAuthoredContentFsRead,
  "no-authored-content-glob": noAuthoredContentGlob,
  "no-direct-astro-content-in-routes": noDirectAstroContentInRoutes,
  "no-runtime-mdx-eval": noRuntimeMdxEval,
  "no-side-loader-imports": noSideLoaderImports
} as const;

const plugin = {
  meta: {
    name: "eslint-plugin-astro-pipeline"
  },
  configs: {
    recommended
  },
  rules
};

export { recommended, rules };
export default plugin;
