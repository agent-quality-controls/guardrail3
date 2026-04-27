use g3ts_astro_seo_types::{
    G3TsAstroSeoEslintPluginContractInput, G3TsAstroSeoEslintSurfaceState,
    G3TsAstroSeoMissingJsonLdHelperInput,
};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-seo/json-ld-helper-rule";
const PLUGIN_PACKAGE_NAME: &str = "g3ts-eslint-plugin-astro-pipeline";
const RULE_NAME: &str = "astro-pipeline/require-approved-json-ld-helper-in-routes";

pub(crate) fn check_missing_source(
    contract: &G3TsAstroSeoMissingJsonLdHelperInput,
    results: &mut Vec<G3CheckResult>,
) {
    results.push(crate::support::error(
        ID,
        "Astro JSON-LD helper source is missing",
        format!(
            "`{}` declares `[ts.astro.seo].json_ld_helpers` path `{}`, but G3TS found no source files there. Configure the approved typed JSON-LD helper module routes may use.",
            contract.policy_rel_path, contract.configured_path
        ),
        Some(&contract.policy_rel_path),
    ));
}

pub(crate) fn check_eslint(
    contract: &G3TsAstroSeoEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = eslint_rel_path(contract);
    if json_ld_rule_effective(contract) {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                ID,
                "Astro JSON-LD helper rule is effective",
                format!("`{rel_path}` enforces `{RULE_NAME}` from `{PLUGIN_PACKAGE_NAME}` on Astro, TS, and TSX lanes with route coverage, endpoint coverage, and non-empty `approvedJsonLdHelperModules`."),
                rel_path,
            ));
        }
        return;
    }

    results.push(error(rel_path));
}

fn eslint_rel_path(contract: &G3TsAstroSeoEslintPluginContractInput) -> Option<&str> {
    match &contract.config {
        G3TsAstroSeoEslintSurfaceState::Missing { rel_path }
        | G3TsAstroSeoEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroSeoEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroSeoEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

fn json_ld_rule_effective(contract: &G3TsAstroSeoEslintPluginContractInput) -> bool {
    let G3TsAstroSeoEslintSurfaceState::Parsed { snapshot } = &contract.config else {
        return false;
    };

    snapshot.astro_source_probe_present
        && snapshot.ts_source_probe_present
        && snapshot.tsx_source_probe_present
        && snapshot
            .astro_source_effective_json_ld_helper_rules
            .iter()
            .any(|rule| rule == RULE_NAME)
        && snapshot
            .ts_source_effective_json_ld_helper_rules
            .iter()
            .any(|rule| rule == RULE_NAME)
        && snapshot
            .tsx_source_effective_json_ld_helper_rules
            .iter()
            .any(|rule| rule == RULE_NAME)
}

fn error(rel_path: Option<&str>) -> G3CheckResult {
    crate::support::error(
        ID,
        "Astro JSON-LD helper rule is not effective",
        format!(
            "`{}` must activate `{RULE_NAME}` from `{PLUGIN_PACKAGE_NAME}` at `error` on Astro, TS, and TSX source probes with `routeGlobs`, `endpointGlobs`, and non-empty `approvedJsonLdHelperModules`. Public route structured data must come through approved typed JSON-LD helpers, not string-built JSON blobs.",
            rel_path.unwrap_or("eslint.config.*")
        ),
        rel_path,
    )
}
