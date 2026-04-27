use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SETUP-CONFIG-07";
const PLUGIN_NAME: &str = "astro-pipeline";
const PLUGIN_PACKAGE_NAME: &str = "g3ts-eslint-plugin-astro-pipeline";
const REQUIRED_RULES: [&str; 9] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
    "astro-pipeline/no-content-data-modules-in-routes",
    "astro-pipeline/no-direct-astro-content-in-routes",
    "astro-pipeline/no-runtime-mdx-eval",
    "astro-pipeline/require-approved-content-adapter-in-routes",
    "astro-pipeline/no-side-loader-imports",
    "astro-pipeline/no-velite-imports",
];
const ROUTE_SCOPED_REQUIRED_RULES: [&str; 8] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
    "astro-pipeline/no-content-data-modules-in-routes",
    "astro-pipeline/no-direct-astro-content-in-routes",
    "astro-pipeline/require-approved-content-adapter-in-routes",
    "astro-pipeline/no-side-loader-imports",
    "astro-pipeline/no-velite-imports",
];
const CONTENT_DATA_REQUIRED_RULES: [&str; 1] = ["astro-pipeline/no-content-data-modules-in-routes"];
const CONTENT_SOURCE_REQUIRED_RULES: [&str; 3] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
];

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.eslint_contracts {
        let rel_path = g3ts_astro_check_support::eslint::eslint_rel_path(contract);
        let has_pipeline_rules =
            g3ts_astro_check_support::eslint::eslint_required_lanes_have_effective_pipeline_rules(
                contract,
                PLUGIN_NAME,
                PLUGIN_PACKAGE_NAME,
                &REQUIRED_RULES,
                &ROUTE_SCOPED_REQUIRED_RULES,
                &CONTENT_DATA_REQUIRED_RULES,
                &CONTENT_SOURCE_REQUIRED_RULES,
            );
        if has_pipeline_rules {
            if let Some(rel_path) = rel_path {
                results.push(g3ts_astro_check_support::core::info(
                    ID,
                    "Astro pipeline ESLint plugin is wired and effective",
                    format!(
                        "`{rel_path}` activates `{PLUGIN_NAME}` from `{PLUGIN_PACKAGE_NAME}` and enforces the required Astro pipeline rules at error severity on the Astro, TS, and TSX source probes. Route-scoped rules cover actual page routes and endpoints; the content-data rule has non-empty `contentDataModuleGlobs`; the authored-content rules have non-empty `authoredContentGlobs` or `specContentGlobs`; and `astro-pipeline/require-approved-content-adapter-in-routes` has non-empty `approvedContentAdapterModules`."
                    ),
                    rel_path,
                ));
            }
            continue;
        }

        let message = match rel_path {
            Some(rel_path) => format!(
                "`{rel_path}` does not activate `{PLUGIN_NAME}` from `{PLUGIN_PACKAGE_NAME}` with all required Astro pipeline rules at error severity on the Astro, TS, and TSX source probes. Import `{PLUGIN_PACKAGE_NAME}`, register it as `{PLUGIN_NAME}`, and enable the required Astro pipeline rules, route coverage for actual Astro page routes, endpoint coverage for actual Astro endpoints, non-empty `contentDataModuleGlobs` on `astro-pipeline/no-content-data-modules-in-routes`, non-empty `authoredContentGlobs` or `specContentGlobs` on the authored-content rules, and non-empty `approvedContentAdapterModules` on `astro-pipeline/require-approved-content-adapter-in-routes`. Without those effective delegated rules, routes can bypass Astro content collections while the package is still installed."
            ),
            None => format!(
                "The Astro pipeline ESLint wiring contract could not be checked because `eslint.config.*` was not available. Restore the app ESLint config, import `{PLUGIN_PACKAGE_NAME}`, register it as `{PLUGIN_NAME}`, and enable the required rules, route coverage for actual Astro page routes, endpoint coverage for actual Astro endpoints, non-empty `contentDataModuleGlobs`, non-empty authored-content globs, and non-empty `approvedContentAdapterModules` on the Astro, TS, and TSX source probes."
            ),
        };
        results.push(g3ts_astro_check_support::core::error(
            ID,
            "Astro ESLint lanes are not enforcing the required content rules",
            message,
            rel_path,
        ));
    }
}
