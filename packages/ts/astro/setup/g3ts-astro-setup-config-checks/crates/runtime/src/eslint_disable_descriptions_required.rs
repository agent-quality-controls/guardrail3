use g3ts_astro_setup_types::{
    G3TsAstroSetupEslintPluginContractInput, G3TsAstroSetupEslintSurfaceState,
};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-setup/eslint-disable-descriptions-required";
const PLUGIN_PACKAGE: &str = "@eslint-community/eslint-plugin-eslint-comments";
const PLUGIN_NAMESPACE: &str = "@eslint-community/eslint-comments";
const RULE_NAME: &str = "@eslint-community/eslint-comments/require-description";

pub(crate) fn check(
    contract: &G3TsAstroSetupEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } = &contract.config else {
        results.push(crate::support::error(
            ID,
            "ESLint disable descriptions cannot be checked",
            "G3TS could not parse the Astro app ESLint effective config. Configure ESLint so the Astro, TS, and TSX source probes load the eslint-comments plugin and `require-description` at error severity.".to_owned(),
            config_rel_path(&contract.config),
        ));
        return;
    };

    if lane_is_configured(
        &snapshot.astro_source_plugin_package_names,
        &snapshot.astro_source_error_rules,
    ) && lane_is_configured(
        &snapshot.ts_source_plugin_package_names,
        &snapshot.ts_source_error_rules,
    ) && lane_is_configured(
        &snapshot.tsx_source_plugin_package_names,
        &snapshot.tsx_source_error_rules,
    ) {
        results.push(crate::support::info(
            ID,
            "ESLint disable descriptions are required on Astro source lanes",
            format!(
                "`{}` loads `{PLUGIN_NAMESPACE}` from `{PLUGIN_PACKAGE}` and enables `{RULE_NAME}` at error severity on Astro, TS, and TSX source probes.",
                snapshot.rel_path
            ),
            &snapshot.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        ID,
        "ESLint disable descriptions are not required on every Astro source lane",
        format!(
            "`{}` must load plugin namespace `{PLUGIN_NAMESPACE}` from package `{PLUGIN_PACKAGE}` and enable `{RULE_NAME}` at `error` on Astro, TS, and TSX source lanes. This makes every `eslint-disable` escape hatch explain why it exists instead of silently bypassing delegated Astro checks.",
            snapshot.rel_path
        ),
        Some(&snapshot.rel_path),
    ));
}

fn lane_is_configured(
    package_names: &std::collections::BTreeMap<String, Vec<String>>,
    error_rules: &[String],
) -> bool {
    package_names
        .get(PLUGIN_NAMESPACE)
        .is_some_and(|names| names.iter().any(|name| name == PLUGIN_PACKAGE))
        && error_rules.iter().any(|rule| rule == RULE_NAME)
}

fn config_rel_path(config: &G3TsAstroSetupEslintSurfaceState) -> Option<&str> {
    match config {
        G3TsAstroSetupEslintSurfaceState::Missing { rel_path }
        | G3TsAstroSetupEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroSetupEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}
