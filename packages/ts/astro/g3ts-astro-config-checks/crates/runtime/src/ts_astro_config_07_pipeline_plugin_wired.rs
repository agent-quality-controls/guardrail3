use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-07";
const PLUGIN_NAME: &str = "astro-pipeline";
const REQUIRED_RULES: [&str; 7] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-content-data-modules-in-routes",
    "astro-pipeline/no-direct-astro-content-in-routes",
    "astro-pipeline/no-runtime-mdx-eval",
    "astro-pipeline/no-side-loader-imports",
    "astro-pipeline/no-velite-imports",
];
const ROUTE_SCOPED_REQUIRED_RULES: [&str; 6] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-content-data-modules-in-routes",
    "astro-pipeline/no-direct-astro-content-in-routes",
    "astro-pipeline/no-side-loader-imports",
    "astro-pipeline/no-velite-imports",
];
const CONTENT_DATA_REQUIRED_RULES: [&str; 1] = ["astro-pipeline/no-content-data-modules-in-routes"];

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.eslint_contracts {
        if !contract.requires_source_pipeline_linting {
            continue;
        }

        let rel_path = crate::support::eslint_rel_path(contract);
        if crate::support::eslint_required_lanes_have_effective_pipeline_rules(
            contract,
            PLUGIN_NAME,
            &REQUIRED_RULES,
            &ROUTE_SCOPED_REQUIRED_RULES,
            &CONTENT_DATA_REQUIRED_RULES,
        ) {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "astro pipeline ESLint plugin wired and effective",
                    format!(
                        "`{rel_path}` activates `{PLUGIN_NAME}` and enforces the required Astro pipeline rules at error severity on the Astro, TS, and TSX source lanes, with route or endpoint scope options present for the route-scoped pipeline rules and non-empty `contentDataModuleGlobs` on the content-data rule."
                    ),
                    rel_path,
                ));
            }
            continue;
        }

        let message = match rel_path {
            Some(rel_path) => format!(
                "`{rel_path}` does not activate `{PLUGIN_NAME}` with all required Astro pipeline rules at error severity on the Astro, TS, and TSX source lanes, with non-empty `routeGlobs` or `endpointGlobs` options on the route-scoped pipeline rules and non-empty `contentDataModuleGlobs` on `astro-pipeline/no-content-data-modules-in-routes`. Enable the `astro-pipeline` plugin, set the required Astro pipeline rules to `error`, and pass non-empty `routeGlobs` or `endpointGlobs` in the route-scoped rule options plus non-empty `contentDataModuleGlobs` on `astro-pipeline/no-content-data-modules-in-routes` in the Astro, TS, and TSX lane configs in `{rel_path}`. Without those options, the route-scoped and content-data pipeline rules stay inert and route bypasses can pass lint silently."
            ),
            None => format!(
                "The Astro pipeline ESLint wiring contract could not be checked because `eslint.config.*` was not available. Restore the app ESLint config and enable `astro-pipeline` with the required rules, non-empty `routeGlobs` or `endpointGlobs` options on the route-scoped rules, and non-empty `contentDataModuleGlobs` on `astro-pipeline/no-content-data-modules-in-routes` on the Astro, TS, and TSX source lanes there. Without those options, the route-scoped and content-data pipeline rules stay inert and route bypasses can pass lint silently."
            ),
        };
        results.push(crate::support::error(
            ID,
            "Astro ESLint lanes are not enforcing the required `astro-pipeline` rules",
            message,
            rel_path,
        ));
    }
}
