use g3ts_astro_mdx_types::{
    G3TsAstroMdxEslintPluginContractInput, G3TsAstroMdxEslintSurfaceState,
    G3TsAstroMdxIntegrationContractInput,
};
use guardrail3_check_types::G3CheckResult;

/// Internal constant `PACKAGE_ID`.
const PACKAGE_ID: &str = "g3ts-astro-mdx/mdx-eslint-plugin-package-present";
/// Internal constant `LANE_ID`.
const LANE_ID: &str = "g3ts-astro-mdx/mdx-eslint-lane-wired";
/// Internal constant `DEPENDENCY_NAME`.
const DEPENDENCY_NAME: &str = "eslint-plugin-mdx";
/// Internal constant `PLUGIN_NAME`.
const PLUGIN_NAME: &str = "mdx";
/// Internal constant `RULE_NAME`.
const RULE_NAME: &str = "mdx/remark";

/// Internal function `check_package`.
pub(crate) fn check_package(
    contract: &G3TsAstroMdxIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    if crate::support::package_has_dependency(&contract.package, DEPENDENCY_NAME) {
        results.push(crate::support::info(
            PACKAGE_ID,
            "MDX ESLint plugin package is installed",
            format!("`{rel_path}` lists `{DEPENDENCY_NAME}`. Astro MDX apps need this package so `.mdx` files run through the `mdx` ESLint plugin namespace."),
            rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        PACKAGE_ID,
        "MDX ESLint plugin package is missing",
        format!(
            "`{rel_path}` must list `{DEPENDENCY_NAME}` in dependencies or devDependencies. Bare `eslint-mdx` is not the app contract because G3TS requires the `mdx` plugin namespace and `{RULE_NAME}` rule."
        ),
        Some(rel_path),
    ));
}

/// Internal function `check_eslint`.
pub(crate) fn check_eslint(
    contract: &G3TsAstroMdxEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = eslint_rel_path(contract);
    if mdx_lane_has_remark_rule(contract) {
        results.push(crate::support::info(
            LANE_ID,
            "MDX ESLint lane is wired",
            format!("`{rel_path}` activates plugin `{PLUGIN_NAME}` and `{RULE_NAME}` at error severity for the MDX content probe."),
            rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        LANE_ID,
        "MDX ESLint lane is not wired",
        format!(
            "`{rel_path}` must have a non-ignored MDX content probe with plugin `{PLUGIN_NAME}` and `{RULE_NAME}` at `error`. Install and configure `{DEPENDENCY_NAME}`; installing bare `eslint-mdx` does not satisfy this rule."
        ),
        Some(rel_path),
    ));
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

/// Internal function `mdx_lane_has_remark_rule`.
fn mdx_lane_has_remark_rule(contract: &G3TsAstroMdxEslintPluginContractInput) -> bool {
    let G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = &contract.config else {
        return false;
    };

    snapshot.mdx_content_probe_present
        && !snapshot.mdx_content_probe_ignored
        && snapshot
            .mdx_content_plugins
            .iter()
            .any(|plugin| plugin == PLUGIN_NAME)
        && snapshot
            .mdx_content_plugin_package_names
            .get(PLUGIN_NAME)
            .is_some_and(|packages| packages.iter().any(|package| package == DEPENDENCY_NAME))
        && snapshot
            .mdx_content_error_rules
            .iter()
            .any(|rule| rule == RULE_NAME)
}
