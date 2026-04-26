use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-31";
const PLUGIN_PACKAGE_NAME: &str = "g3ts-eslint-plugin-astro-pipeline";
const RULE_NAME: &str = "astro-pipeline/require-approved-metadata-helper-in-routes";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let policy_rel_path = crate::support::astro_policy_rel_path(contract);
        let Some(policy) = crate::support::parsed_astro_policy(contract) else {
            continue;
        };

        if !contract
            .approved_surface_sources
            .missing_metadata_helpers
            .is_empty()
        {
            results.push(crate::support::error(
                ID,
                "Astro metadata helper sources are missing",
                format!(
                    "`{}` declares `metadata_helpers = [{}]`, but G3TS found no source files at those app-relative paths. Configure the approved metadata helper modules routes may use.",
                    policy.rel_path,
                    contract.approved_surface_sources.missing_metadata_helpers.join(", ")
                ),
                policy_rel_path,
            ));
        }
    }

    for contract in &input.eslint_contracts {
        let rel_path = crate::support::eslint_rel_path(contract);
        let Some(eslint) = crate::support::parsed_eslint_surface(contract) else {
            results.push(error(rel_path));
            continue;
        };

        if [&eslint.astro_source_effective_metadata_helper_rules, &eslint.ts_source_effective_metadata_helper_rules, &eslint.tsx_source_effective_metadata_helper_rules]
            .iter()
            .all(|rules| rules.iter().any(|rule| rule == RULE_NAME))
        {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "Astro metadata helper rule is effective",
                    format!("`{rel_path}` enforces `{RULE_NAME}` from `{PLUGIN_PACKAGE_NAME}` on Astro, TS, and TSX lanes with route coverage, endpoint coverage, non-empty `approvedMetadataHelperModules`, and non-empty `approvedContentAdapterModules`."),
                    rel_path,
                ));
            }
            continue;
        }

        results.push(error(rel_path));
    }
}

fn error(rel_path: Option<&str>) -> G3CheckResult {
    crate::support::error(
        ID,
        "Astro metadata helper rule is not effective",
        format!(
            "`{}` must activate `{RULE_NAME}` from `{PLUGIN_PACKAGE_NAME}` at `error` on Astro, TS, and TSX source probes with `routeGlobs`, `endpointGlobs`, non-empty `approvedMetadataHelperModules`, and non-empty `approvedContentAdapterModules`. Public route metadata must come through approved typed surfaces, not hardcoded page/layout defaults.",
            rel_path.unwrap_or("eslint.config.*")
        ),
        rel_path,
    )
}
