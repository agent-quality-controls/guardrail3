use g3ts_astro_setup_types::{
    G3TsAstroSetupEslintPluginContractInput, G3TsAstroSetupEslintSurfaceState,
};
use guardrail3_check_types::G3CheckResult;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3ts-astro-setup/astro-eslint-plugin-wired";
/// Required npm dependency name.
const DEPENDENCY_NAME: &str = "eslint-plugin-astro";
/// Static rule data.
const PLUGIN_NAME: &str = "astro";
/// Static rule data.
const RULE_NAME: &str = "astro/valid-compile";

/// Validates the rule and pushes findings into `results`.
pub(crate) fn check(
    contract: &G3TsAstroSetupEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = eslint_rel_path(contract);
    if astro_source_has_plugin(contract, PLUGIN_NAME) {
        results.push(crate::support::info(
            ID,
            "astro ESLint plugin wired",
            format!("`{rel_path}` activates `{PLUGIN_NAME}` for the required Astro source probe."),
            rel_path,
        ));
        return;
    }

    let message = format!(
        "`{rel_path}` does not activate `{PLUGIN_NAME}` from `eslint-plugin-astro` on the required Astro source probe. Add the `astro` plugin from `eslint-plugin-astro` to the Astro file lane in `{rel_path}`. Astro files must run through the Astro plugin so framework lint rules actually execute."
    );
    results.push(crate::support::error(
        ID,
        "Astro ESLint source probe is not wired to `eslint-plugin-astro`",
        message,
        Some(rel_path),
    ));
}

/// Internal helper used by the rule.
fn eslint_rel_path(contract: &G3TsAstroSetupEslintPluginContractInput) -> &str {
    match &contract.config {
        G3TsAstroSetupEslintSurfaceState::Missing { rel_path }
        | G3TsAstroSetupEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroSetupEslintSurfaceState::ParseError { rel_path, .. } => rel_path,
        G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } => &snapshot.rel_path,
    }
}

/// Internal helper used by the rule.
fn astro_source_has_plugin(
    contract: &G3TsAstroSetupEslintPluginContractInput,
    plugin_name: &str,
) -> bool {
    let G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } = &contract.config else {
        return false;
    };

    snapshot.astro_source_probe_present
        && !snapshot.astro_source_probe_ignored
        && snapshot
            .astro_source_plugins
            .iter()
            .any(|plugin| plugin == plugin_name)
        && snapshot
            .astro_source_plugin_package_names
            .get(plugin_name)
            .is_some_and(|packages| packages.iter().any(|package| package == DEPENDENCY_NAME))
        && snapshot
            .astro_source_error_rules
            .iter()
            .any(|rule| rule == RULE_NAME)
}
