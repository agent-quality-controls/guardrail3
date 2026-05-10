use g3ts_astro_content_types::{
    G3TsAstroContentEslintPluginContractInput, G3TsAstroContentEslintSurfaceState,
    G3TsAstroContentIntegrationContractInput,
};
use guardrail3_check_types::G3CheckResult;

/// Internal constant `ID`.
const ID: &str = "g3ts-astro-content/inline-copy-rule";
/// Internal constant `DEPENDENCY_NAME`.
const DEPENDENCY_NAME: &str = "eslint-plugin-i18next";
/// Internal constant `PLUGIN_NAME`.
const PLUGIN_NAME: &str = "i18next";
/// Internal constant `RULE_NAME`.
const RULE_NAME: &str = "i18next/no-literal-string";

/// Internal function `check_package`.
pub(crate) fn check_package(
    contract: &G3TsAstroContentIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    if !crate::support::package_has_dependency(&contract.package, DEPENDENCY_NAME) {
        results.push(crate::support::error(
            ID,
            "Inline public-copy ESLint plugin package is missing",
            format!(
                "`{rel_path}` must list `{DEPENDENCY_NAME}` in dependencies or devDependencies. The Astro family delegates hardcoded public-copy detection to `{RULE_NAME}`."
            ),
            Some(rel_path),
        ));
    }
}

/// Internal function `check_eslint`.
pub(crate) fn check_eslint(
    contract: &G3TsAstroContentEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = eslint_rel_path(contract);
    if required_lanes_have_inline_public_content_rule(contract) {
        results.push(crate::support::info(
            ID,
            "Inline public-copy ESLint rule is effective",
            format!("`{rel_path}` enforces `{RULE_NAME}` with the exact strict Astro public-copy options on Astro, TS, and TSX source probes."),
            rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        ID,
        "Inline public-copy ESLint rule is not effective",
        format!(
            "`{rel_path}` must activate plugin `{PLUGIN_NAME}` and rule `{RULE_NAME}` at `error` on Astro, TS, and TSX source probes with the exact strict options from the Astro delegation plan. Missing probes, ignored probes, broad allowlists, or changed messages fail this contract."
        ),
        Some(rel_path),
    ));
}

/// Returns the relative path of the eslint config across all parse states.
fn eslint_rel_path(contract: &G3TsAstroContentEslintPluginContractInput) -> &str {
    match &contract.config {
        G3TsAstroContentEslintSurfaceState::Missing { rel_path }
        | G3TsAstroContentEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroContentEslintSurfaceState::ParseError { rel_path, .. } => rel_path,
        G3TsAstroContentEslintSurfaceState::Parsed { snapshot } => &snapshot.rel_path,
    }
}

/// Internal function `required_lanes_have_inline_public_content_rule`.
fn required_lanes_have_inline_public_content_rule(
    contract: &G3TsAstroContentEslintPluginContractInput,
) -> bool {
    let G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = &contract.config else {
        return false;
    };

    lane_has_inline_public_content_rule(
        snapshot.astro_source_probe_present,
        snapshot.astro_source_probe_ignored,
        &snapshot.astro_source_plugins,
        &snapshot.astro_source_error_rules,
        &snapshot.astro_source_effective_inline_public_content_rules,
    ) && lane_has_inline_public_content_rule(
        snapshot.ts_source_probe_present,
        snapshot.ts_source_probe_ignored,
        &snapshot.ts_source_plugins,
        &snapshot.ts_source_error_rules,
        &snapshot.ts_source_effective_inline_public_content_rules,
    ) && lane_has_inline_public_content_rule(
        snapshot.tsx_source_probe_present,
        snapshot.tsx_source_probe_ignored,
        &snapshot.tsx_source_plugins,
        &snapshot.tsx_source_error_rules,
        &snapshot.tsx_source_effective_inline_public_content_rules,
    )
}

/// Internal function `lane_has_inline_public_content_rule`.
fn lane_has_inline_public_content_rule(
    probe_present: bool,
    probe_ignored: bool,
    plugins: &[String],
    error_rules: &[String],
    effective_rules: &[String],
) -> bool {
    probe_present
        && !probe_ignored
        && plugins.iter().any(|plugin| plugin == PLUGIN_NAME)
        && error_rules.iter().any(|rule| rule == RULE_NAME)
        && effective_rules.iter().any(|rule| rule == RULE_NAME)
}
