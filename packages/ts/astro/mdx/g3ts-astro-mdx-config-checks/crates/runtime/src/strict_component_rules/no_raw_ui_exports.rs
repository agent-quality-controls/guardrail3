use g3ts_astro_mdx_types::{G3TsAstroMdxEslintPluginContractInput, G3TsAstroMdxEslintSurfaceState};
use guardrail3_check_types::G3CheckResult;

/// Internal constant `ID`.
const ID: &str = "g3ts-astro-mdx/no-raw-ui-exports";
/// Internal constant `RULE_NAME`.
const RULE_NAME: &str = "astro-pipeline/mdx-component-map-no-raw-ui-exports";

/// Internal function `check_eslint`.
pub(crate) fn check_eslint(
    contract: &G3TsAstroMdxEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = eslint_rel_path(contract);
    if effective(contract) {
        results.push(crate::support::info(
            ID,
            "Astro MDX component-map raw UI export rule is effective",
            format!("`{rel_path}` enforces `{RULE_NAME}` at `error` on the approved component-map lane with explicit raw UI module globs."),
            rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        ID,
        "Astro MDX component-map raw UI export rule is not effective",
        format!(
            "`{rel_path}` must activate `{RULE_NAME}` from `g3ts-eslint-plugin-astro-pipeline` at `error` on the configured `[ts.astro.mdx].component_maps` lane with non-empty `approvedMdxComponentModules` and `rawUiModuleGlobs`. Component maps may wrap raw UI, but must not export raw UI components directly."
        ),
        Some(rel_path),
    ));
}

/// Internal function `effective`.
fn effective(contract: &G3TsAstroMdxEslintPluginContractInput) -> bool {
    let G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = &contract.config else {
        return false;
    };

    snapshot.component_map_probe_present
        && !snapshot.component_map_probe_ignored
        && snapshot
            .component_map_effective_no_raw_ui_export_rules
            .iter()
            .any(|rule| rule == RULE_NAME)
}

/// Internal function `eslint_rel_path`.
fn eslint_rel_path(contract: &G3TsAstroMdxEslintPluginContractInput) -> &str {
    match &contract.config {
        G3TsAstroMdxEslintSurfaceState::Missing { rel_path }
        | G3TsAstroMdxEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroMdxEslintSurfaceState::ParseError { rel_path, .. } => rel_path,
        G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } => &snapshot.rel_path,
    }
}
