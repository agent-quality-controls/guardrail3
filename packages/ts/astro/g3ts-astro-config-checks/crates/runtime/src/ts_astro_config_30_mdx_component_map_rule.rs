use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-30";
const PLUGIN_PACKAGE_NAME: &str = "g3ts-eslint-plugin-astro-pipeline";
const RULE_NAME: &str = "astro-pipeline/mdx-component-imports-from-approved-map";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let policy_rel_path = crate::support::astro_policy_rel_path(contract);
        let Some(policy) = crate::support::parsed_astro_policy(contract) else {
            continue;
        };

        if !contract
            .approved_surface_sources
            .missing_mdx_component_maps
            .is_empty()
        {
            results.push(crate::support::error(
                ID,
                "Astro MDX component-map sources are missing",
                format!(
                    "`{}` declares `[ts.astro.mdx].component_maps = [{}]`, but G3TS found no source files at those app-relative paths. Configure the approved MDX component-map modules that MDX files may import.",
                    policy.rel_path,
                    contract.approved_surface_sources.missing_mdx_component_maps.join(", ")
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

        if eslint
            .mdx_content_effective_mdx_component_map_rules
            .iter()
            .any(|rule| rule == RULE_NAME)
        {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "Astro MDX component-map rule is effective",
                    format!("`{rel_path}` enforces `{RULE_NAME}` from `{PLUGIN_PACKAGE_NAME}` on the MDX content lane with non-empty `mdxContentGlobs` and `approvedMdxComponentModules`."),
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
        "Astro MDX component-map rule is not effective",
        format!(
            "`{}` must activate `{RULE_NAME}` from `{PLUGIN_PACKAGE_NAME}` at `error` on the MDX content probe with non-empty `mdxContentGlobs` and `approvedMdxComponentModules` derived from `[ts.astro.mdx].component_maps`. MDX component imports must come only from approved component-map modules.",
            rel_path.unwrap_or("eslint.config.*")
        ),
        rel_path,
    )
}
