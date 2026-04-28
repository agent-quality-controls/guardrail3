use g3ts_astro_mdx_types::{G3TsAstroMdxEslintPluginContractInput, G3TsAstroMdxEslintSurfaceState};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-mdx/eslint-disable-descriptions-required";
const PLUGIN_PACKAGE: &str = "@eslint-community/eslint-plugin-eslint-comments";
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
            "G3TS could not parse the Astro MDX ESLint effective config. Configure the eslint-comments plugin and `require-description` at error severity on MDX content and component-map probes.".to_owned(),
            config_rel_path(&contract.config),
        ));
        return;
    };

    if plugin_is_loaded(&snapshot.mdx_content_plugin_package_names)
        && plugin_is_loaded(&snapshot.component_map_plugin_package_names)
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
            "`{}` must load `{PLUGIN_PACKAGE}` as `{PLUGIN_NAMESPACE}` and enable `{RULE_NAME}` at `error` on MDX content and component-map probes. Every MDX disable escape hatch must carry a description.",
            snapshot.rel_path
        ),
        Some(&snapshot.rel_path),
    ));
}

fn plugin_is_loaded(packages: &std::collections::BTreeMap<String, Vec<String>>) -> bool {
    packages
        .get(PLUGIN_NAMESPACE)
        .is_some_and(|names| names.iter().any(|name| name == PLUGIN_PACKAGE))
}

fn config_rel_path(config: &G3TsAstroMdxEslintSurfaceState) -> Option<&str> {
    match config {
        G3TsAstroMdxEslintSurfaceState::Missing { rel_path }
        | G3TsAstroMdxEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroMdxEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}
