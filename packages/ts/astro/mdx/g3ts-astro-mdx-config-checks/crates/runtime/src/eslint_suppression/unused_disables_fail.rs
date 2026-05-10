use g3ts_astro_mdx_types::{G3TsAstroMdxEslintPluginContractInput, G3TsAstroMdxEslintSurfaceState};
use guardrail3_check_types::G3CheckResult;

/// Internal constant `ID`.
const ID: &str = "g3ts-astro-mdx/unused-eslint-disables-fail";

/// Internal function `check`.
pub(crate) fn check(
    contract: &G3TsAstroMdxEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = &contract.config else {
        results.push(crate::support::error(
            ID,
            "MDX unused-disable policy cannot be checked",
            "G3TS could not parse the Astro MDX ESLint effective config. Configure unused-disable reporting at error severity on MDX content and component-map probes.".to_owned(),
            Some(config_rel_path(&contract.config)),
        ));
        return;
    };

    if snapshot.mdx_content_unused_disable_fail_closed
        && snapshot.component_map_unused_disable_fail_closed
    {
        results.push(crate::support::info(
            ID,
            "MDX unused ESLint disables fail validation",
            format!("`{}` fails closed for stale ESLint disable directives on MDX content and component-map probes.", snapshot.rel_path),
            &snapshot.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        ID,
        "MDX unused ESLint disables do not fail validation",
        format!(
            "`{}` must either enable `@eslint-community/eslint-comments/no-unused-disable` at `error` or configure ESLint core unused-disable reporting at `error` on MDX content and component-map probes. Stale MDX disables otherwise hide bypasses after violations disappear.",
            snapshot.rel_path
        ),
        Some(&snapshot.rel_path),
    ));
}

/// Internal function `config_rel_path`.
fn config_rel_path(config: &G3TsAstroMdxEslintSurfaceState) -> &str {
    match config {
        G3TsAstroMdxEslintSurfaceState::Missing { rel_path }
        | G3TsAstroMdxEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroMdxEslintSurfaceState::ParseError { rel_path, .. } => rel_path,
        G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } => &snapshot.rel_path,
    }
}
