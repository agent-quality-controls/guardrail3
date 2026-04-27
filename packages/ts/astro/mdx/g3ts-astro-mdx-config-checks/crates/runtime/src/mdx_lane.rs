use g3ts_astro_mdx_types::{
    G3TsAstroMdxEslintPluginContractInput, G3TsAstroMdxEslintSurfaceState,
    G3TsAstroMdxIntegrationContractInput,
};
use guardrail3_check_types::G3CheckResult;

const PACKAGE_ID: &str = "g3ts-astro-mdx/mdx-eslint-plugin-package-present";
const LANE_ID: &str = "g3ts-astro-mdx/mdx-eslint-lane-wired";
const DEPENDENCY_NAME: &str = "eslint-plugin-mdx";
const PLUGIN_NAME: &str = "mdx";
const RULE_NAME: &str = "mdx/remark";

pub(crate) fn check_package(
    contract: &G3TsAstroMdxIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    if !crate::support::package_has_dependency(&contract.package, DEPENDENCY_NAME) {
        results.push(crate::support::error(
            PACKAGE_ID,
            "MDX ESLint plugin package is missing",
            format!(
                "`{}` must list `{DEPENDENCY_NAME}` in dependencies or devDependencies. Bare `eslint-mdx` is not the app contract because G3TS requires the `mdx` plugin namespace and `{RULE_NAME}` rule.",
                rel_path.unwrap_or("package.json")
            ),
            rel_path,
        ));
    }
}

pub(crate) fn check_eslint(
    contract: &G3TsAstroMdxEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = eslint_rel_path(contract);
    if mdx_lane_has_remark_rule(contract) {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                LANE_ID,
                "MDX ESLint lane is wired",
                format!("`{rel_path}` activates plugin `{PLUGIN_NAME}` and `{RULE_NAME}` at error severity for the MDX content probe."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        LANE_ID,
        "MDX ESLint lane is not wired",
        format!(
            "`{}` must have a non-ignored MDX content probe with plugin `{PLUGIN_NAME}` and `{RULE_NAME}` at `error`. Install and configure `{DEPENDENCY_NAME}`; installing bare `eslint-mdx` does not satisfy this rule.",
            rel_path.unwrap_or("eslint.config.*")
        ),
        rel_path,
    ));
}

fn eslint_rel_path(contract: &G3TsAstroMdxEslintPluginContractInput) -> Option<&str> {
    match &contract.config {
        G3TsAstroMdxEslintSurfaceState::Missing { rel_path }
        | G3TsAstroMdxEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroMdxEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

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
