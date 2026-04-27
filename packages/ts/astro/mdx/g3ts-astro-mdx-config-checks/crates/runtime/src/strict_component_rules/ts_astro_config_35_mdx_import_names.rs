use g3ts_astro_mdx_types::{
    G3TsAstroMdxEslintPluginContractInput, G3TsAstroMdxEslintSurfaceState,
};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-MDX-CONFIG-35";
const RULE_NAME: &str = "astro-pipeline/mdx-imports-only-approved-components";

pub(crate) fn check_eslint(
    contract: &G3TsAstroMdxEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = eslint_rel_path(contract);
    if effective(contract) {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                ID,
                "Astro MDX import-name rule is effective",
                format!("`{rel_path}` enforces `{RULE_NAME}` at `error` on the MDX content lane with explicit approved component-map modules and names."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        ID,
        "Astro MDX import-name rule is not effective",
        format!(
            "`{}` must activate `{RULE_NAME}` from `g3ts-eslint-plugin-astro-pipeline` at `error` on the MDX content lane with non-empty `mdxContentGlobs`, `approvedMdxComponentModules`, and `approvedMdxComponentNames`. MDX files must import only explicit validated component-map exports.",
            rel_path.unwrap_or("eslint.config.*")
        ),
        rel_path,
    ));
}

fn effective(contract: &G3TsAstroMdxEslintPluginContractInput) -> bool {
    let G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = &contract.config else {
        return false;
    };

    snapshot.mdx_content_probe_present
        && !snapshot.mdx_content_probe_ignored
        && snapshot
            .mdx_content_effective_named_component_import_rules
            .iter()
            .any(|rule| rule == RULE_NAME)
}

fn eslint_rel_path(contract: &G3TsAstroMdxEslintPluginContractInput) -> Option<&str> {
    match &contract.config {
        G3TsAstroMdxEslintSurfaceState::Missing { rel_path }
        | G3TsAstroMdxEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroMdxEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}
