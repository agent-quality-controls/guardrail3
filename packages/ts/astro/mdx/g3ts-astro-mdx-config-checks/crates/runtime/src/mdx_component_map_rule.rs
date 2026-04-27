use g3ts_astro_mdx_types::{
    G3TsAstroMdxEslintPluginContractInput, G3TsAstroMdxEslintSurfaceState,
    G3TsAstroMdxMissingComponentMapInput,
};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-mdx/mdx-component-map-rule";
const PLUGIN_PACKAGE_NAME: &str = "g3ts-eslint-plugin-astro-pipeline";
const RULE_NAME: &str = "astro-pipeline/mdx-component-imports-from-approved-map";

pub(crate) fn check_missing_source(
    contract: &G3TsAstroMdxMissingComponentMapInput,
    results: &mut Vec<G3CheckResult>,
) {
    results.push(crate::support::error(
        ID,
        "Astro MDX component-map source is missing",
        format!(
            "`{}` declares `[ts.astro.mdx].component_maps` path `{}`, but G3TS found no source files there. Configure the approved MDX component-map module that MDX files may import.",
            contract.policy_rel_path, contract.configured_path
        ),
        Some(&contract.policy_rel_path),
    ));
}

pub(crate) fn check_eslint(
    contract: &G3TsAstroMdxEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = eslint_rel_path(contract);
    if mdx_component_map_rule_effective(contract) {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                ID,
                "Astro MDX component-map rule is effective",
                format!("`{rel_path}` enforces `{RULE_NAME}` from `{PLUGIN_PACKAGE_NAME}` on the MDX content lane with non-empty `mdxContentGlobs` and `approvedMdxComponentModules`."),
                rel_path,
            ));
        }
        return;
    }

    results.push(error(rel_path));
}

fn eslint_rel_path(contract: &G3TsAstroMdxEslintPluginContractInput) -> Option<&str> {
    match &contract.config {
        G3TsAstroMdxEslintSurfaceState::Missing { rel_path }
        | G3TsAstroMdxEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroMdxEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

fn mdx_component_map_rule_effective(contract: &G3TsAstroMdxEslintPluginContractInput) -> bool {
    let G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = &contract.config else {
        return false;
    };

    snapshot.mdx_content_probe_present
        && !snapshot.mdx_content_probe_ignored
        && snapshot
            .mdx_content_effective_mdx_component_map_rules
            .iter()
            .any(|rule| rule == RULE_NAME)
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
