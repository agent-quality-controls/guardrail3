use g3ts_astro_setup_types::{
    G3TsAstroSetupEslintPluginContractInput, G3TsAstroSetupEslintSurfaceState,
};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-setup/astro-eslint-plugin-wired";
const DEPENDENCY_NAME: &str = "eslint-plugin-astro";
const PLUGIN_NAME: &str = "astro";
const RULE_NAME: &str = "astro/valid-compile";

pub(crate) fn check(
    contract: &G3TsAstroSetupEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = eslint_rel_path(contract);
    if astro_source_has_plugin(contract, PLUGIN_NAME) {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                ID,
                "astro ESLint plugin wired",
                format!(
                    "`{rel_path}` activates `{PLUGIN_NAME}` for the required Astro source probe."
                ),
                rel_path,
            ));
        }
        return;
    }

    let message = match rel_path {
            Some(rel_path) => format!(
                "`{rel_path}` does not activate `{PLUGIN_NAME}` from `eslint-plugin-astro` on the required Astro source probe. Add the `astro` plugin from `eslint-plugin-astro` to the Astro file lane in `{rel_path}`. Astro files must run through the Astro plugin so framework lint rules actually execute."
            ),
            None => "The Astro ESLint wiring contract could not be checked because `eslint.config.*` was not available. Restore the app ESLint config and wire the `astro` plugin from `eslint-plugin-astro` for Astro files there. Astro files must run through the Astro plugin so framework lint rules actually execute.".to_owned(),
        };
    results.push(crate::support::error(
        ID,
        "Astro ESLint source probe is not wired to `eslint-plugin-astro`",
        message,
        rel_path,
    ));
}

fn eslint_rel_path(contract: &G3TsAstroSetupEslintPluginContractInput) -> Option<&str> {
    match &contract.config {
        G3TsAstroSetupEslintSurfaceState::Missing { rel_path }
        | G3TsAstroSetupEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroSetupEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

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
