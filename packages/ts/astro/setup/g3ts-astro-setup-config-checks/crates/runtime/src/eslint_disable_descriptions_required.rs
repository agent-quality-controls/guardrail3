use g3ts_astro_setup_types::{
    G3TsAstroSetupEslintPluginContractInput, G3TsAstroSetupEslintSurfaceState,
};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-setup/eslint-disable-descriptions-required";
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
            "G3TS could not parse the Astro app ESLint effective config. Configure ESLint so the Astro, TS, and TSX source probes load the eslint-comments plugin namespace and `require-description` at error severity.".to_owned(),
            config_rel_path(&contract.config),
        ));
        return;
    };

    if lane_is_configured(
        &snapshot.astro_source_plugins,
        &snapshot.astro_source_error_rules,
    ) && lane_is_configured(&snapshot.ts_source_plugins, &snapshot.ts_source_error_rules)
        && lane_is_configured(
            &snapshot.tsx_source_plugins,
            &snapshot.tsx_source_error_rules,
        )
    {
        results.push(crate::support::info(
            ID,
            "ESLint disable descriptions are required on Astro source lanes",
            format!(
                "`{}` loads `{PLUGIN_NAMESPACE}` and enables `{RULE_NAME}` at error severity on Astro, TS, and TSX source probes.",
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
            "`{}` must load plugin namespace `{PLUGIN_NAMESPACE}` and enable `{RULE_NAME}` at `error` on Astro, TS, and TSX source lanes. This makes every `eslint-disable` escape hatch explain why it exists instead of silently bypassing delegated Astro checks.",
            snapshot.rel_path
        ),
        Some(&snapshot.rel_path),
    ));
}

fn lane_is_configured(plugins: &[String], error_rules: &[String]) -> bool {
    plugins.iter().any(|plugin| plugin == PLUGIN_NAMESPACE)
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
