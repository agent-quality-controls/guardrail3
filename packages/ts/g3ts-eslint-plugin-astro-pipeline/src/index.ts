import recommended from "./configs/recommended.js";
import noAuthoredContentFsRead from "./rules/no-authored-content-fs-read.js";
import noAuthoredContentGlob from "./rules/no-authored-content-glob.js";
import noAuthoredContentImports from "./rules/no-authored-content-imports.js";
import noContentDataModulesInRoutes from "./rules/no-content-data-modules-in-routes.js";
import noDirectAstroContentInRoutes from "./rules/no-direct-astro-content-in-routes.js";
import noRuntimeMdxEval from "./rules/no-runtime-mdx-eval.js";
import noSideLoaderImports from "./rules/no-side-loader-imports.js";
import noVeliteImports from "./rules/no-velite-imports.js";
import requireApprovedContentAdapterInRoutes from "./rules/require-approved-content-adapter-in-routes.js";

const rules = {
  "no-authored-content-fs-read": noAuthoredContentFsRead,
  "no-authored-content-glob": noAuthoredContentGlob,
  "no-authored-content-imports": noAuthoredContentImports,
  "no-content-data-modules-in-routes": noContentDataModulesInRoutes,
  "no-direct-astro-content-in-routes": noDirectAstroContentInRoutes,
  "no-runtime-mdx-eval": noRuntimeMdxEval,
  "no-side-loader-imports": noSideLoaderImports,
  "no-velite-imports": noVeliteImports,
  "require-approved-content-adapter-in-routes": requireApprovedContentAdapterInRoutes
} as const;

const plugin = {
  meta: {
    name: "g3ts-eslint-plugin-astro-pipeline"
  },
  configs: {
    recommended
  },
  rules
};

export { recommended, rules };
export default plugin;
