use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-18";
const PLUGIN_NAME: &str = "astro-pipeline";
const PLUGIN_PACKAGE_NAME: &str = "g3ts-eslint-plugin-astro-pipeline";
const RULE_NAME: &str = "astro-pipeline/require-approved-content-adapter-in-routes";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.eslint_contracts {
        let rel_path = crate::support::eslint_rel_path(contract);
        if crate::support::eslint_required_lanes_have_effective_pipeline_rules(
            contract,
            PLUGIN_NAME,
            PLUGIN_PACKAGE_NAME,
            &[RULE_NAME],
            &[RULE_NAME],
            &[],
            &[],
        ) {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "Astro content adapter route rule is effective",
                    format!("`{rel_path}` enforces `{RULE_NAME}` from `{PLUGIN_PACKAGE_NAME}` with route coverage, endpoint coverage, and non-empty `approvedContentAdapterModules` on Astro, TS, and TSX source probes."),
                    rel_path,
                ));
            }
            continue;
        }

        results.push(crate::support::error(
            ID,
            "Astro content adapter route rule is not effective",
            format!(
                "`{}` must import `{PLUGIN_PACKAGE_NAME}`, register it as `{PLUGIN_NAME}`, and activate rule `{RULE_NAME}` at `error` on Astro, TS, and TSX source probes with `routeGlobs`, `endpointGlobs`, and non-empty `approvedContentAdapterModules`. Public page routes must import an approved content adapter instead of reading content directly.",
                rel_path.unwrap_or("eslint.config.*")
            ),
            rel_path,
        ));
    }
}
