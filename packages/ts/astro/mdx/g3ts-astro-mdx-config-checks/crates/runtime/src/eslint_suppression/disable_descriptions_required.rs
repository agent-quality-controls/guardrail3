use g3ts_astro_mdx_types::{G3TsAstroMdxEslintPluginContractInput, G3TsAstroMdxEslintSurfaceState};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-mdx/eslint-disable-descriptions-required";
const PLUGIN_NAMESPACE: &str = "@eslint-community/eslint-comments";
const RULE_NAME: &str = "@eslint-community/eslint-comments/require-description";

pub(crate) fn check(
    contract: &G3TsAstroMdxEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = &contract.config else {
        results.push(crate::support::error(
            ID,
            "MDX ESLint disable description policy cannot be checked",
            "G3TS could not parse the Astro MDX ESLint effective config. Configure the eslint-comments plugin namespace and `require-description` at error severity on MDX content and component-map probes.".to_owned(),
            config_rel_path(&contract.config),
        ));
        return;
    };

    if snapshot
        .mdx_content_plugins
        .iter()
        .any(|plugin| plugin == PLUGIN_NAMESPACE)
        && snapshot
            .component_map_plugins
            .iter()
            .any(|plugin| plugin == PLUGIN_NAMESPACE)
        && snapshot
            .mdx_content_error_rules
            .iter()
            .any(|rule| rule == RULE_NAME)
        && snapshot
            .component_map_error_rules
            .iter()
            .any(|rule| rule == RULE_NAME)
    {
        results.push(crate::support::info(
            ID,
            "MDX ESLint disables require descriptions",
            format!("`{}` requires descriptions for ESLint disable directives on MDX content and component-map probes.", snapshot.rel_path),
            &snapshot.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        ID,
        "MDX ESLint disables can be hidden",
        format!(
            "`{}` must load plugin namespace `{PLUGIN_NAMESPACE}` and enable `{RULE_NAME}` at `error` on MDX content and component-map probes. Every MDX disable escape hatch must carry a description.",
            snapshot.rel_path
        ),
        Some(&snapshot.rel_path),
    ));
}

fn config_rel_path(config: &G3TsAstroMdxEslintSurfaceState) -> Option<&str> {
    match config {
        G3TsAstroMdxEslintSurfaceState::Missing { rel_path }
        | G3TsAstroMdxEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroMdxEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}
