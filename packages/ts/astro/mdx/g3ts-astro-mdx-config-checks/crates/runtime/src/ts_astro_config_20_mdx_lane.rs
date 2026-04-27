use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-MDX-CONFIG-20";
const DEPENDENCY_NAME: &str = "eslint-plugin-mdx";
const PLUGIN_NAME: &str = "mdx";
const RULE_NAME: &str = "mdx/remark";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = g3ts_astro_check_support::core::package_rel_path(contract);
        if !g3ts_astro_check_support::core::package_has_dependency(contract, DEPENDENCY_NAME) {
            results.push(g3ts_astro_check_support::core::error(
                ID,
                "MDX ESLint plugin package is missing",
                format!(
                    "`{}` must list `{DEPENDENCY_NAME}` in dependencies or devDependencies. Bare `eslint-mdx` is not the app contract because G3TS requires the `mdx` plugin namespace and `{RULE_NAME}` rule.",
                    rel_path.unwrap_or("package.json")
                ),
                rel_path,
            ));
        }
    }

    for contract in &input.eslint_contracts {
        let rel_path = g3ts_astro_check_support::eslint::eslint_rel_path(contract);
        if g3ts_astro_check_support::eslint::eslint_mdx_lane_has_remark_rule(
            contract,
            PLUGIN_NAME,
            RULE_NAME,
        ) {
            if let Some(rel_path) = rel_path {
                results.push(g3ts_astro_check_support::core::info(
                    ID,
                    "MDX ESLint lane is wired",
                    format!("`{rel_path}` activates plugin `{PLUGIN_NAME}` and `{RULE_NAME}` at error severity for the MDX content probe."),
                    rel_path,
                ));
            }
            continue;
        }

        results.push(g3ts_astro_check_support::core::error(
            ID,
            "MDX ESLint lane is not wired",
            format!(
                "`{}` must have a non-ignored MDX content probe with plugin `{PLUGIN_NAME}` and `{RULE_NAME}` at `error`. Install and configure `{DEPENDENCY_NAME}`; installing bare `eslint-mdx` does not satisfy this rule.",
                rel_path.unwrap_or("eslint.config.*")
            ),
            rel_path,
        ));
    }
}
