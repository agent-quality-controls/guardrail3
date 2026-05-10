use g3ts_astro_mdx_types::{G3TsAstroMdxEslintPluginContractInput, G3TsAstroMdxEslintSurfaceState};
use guardrail3_check_types::G3CheckResult;

/// Internal constant `ID`.
const ID: &str = "g3ts-astro-mdx/protected-mdx-rule-disables-restricted";
/// Internal constant `RESTRICT_RULE`.
const RESTRICT_RULE: &str = "@eslint-community/eslint-comments/no-restricted-disable";
/// Internal constant `MDX_CONTENT_PROTECTED_RULES`.
const MDX_CONTENT_PROTECTED_RULES: [&str; 4] = [
    "mdx/remark",
    "astro-pipeline/mdx-component-imports-from-approved-map",
    "astro-pipeline/mdx-imports-only-approved-components",
    "astro-pipeline/no-raw-mdx-images",
];
/// Internal constant `COMPONENT_MAP_PROTECTED_RULES`.
const COMPONENT_MAP_PROTECTED_RULES: [&str; 2] = [
    "astro-pipeline/mdx-component-map-no-raw-ui-exports",
    "astro-pipeline/mdx-component-wrapper-requires-zod-parse",
];

/// Internal function `check`.
pub(crate) fn check(
    contract: &G3TsAstroMdxEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = &contract.config else {
        results.push(crate::support::error(
            ID,
            "MDX protected-disable policy cannot be checked",
            "G3TS could not parse the Astro MDX ESLint effective config. Configure `@eslint-community/eslint-comments/no-restricted-disable` for MDX content and MDX component-map probes.".to_owned(),
            Some(config_rel_path(&contract.config)),
        ));
        return;
    };

    if lane_is_restricted(
        &snapshot.mdx_content_warn_or_error_rules,
        &snapshot.mdx_content_restricted_disable_patterns,
        &MDX_CONTENT_PROTECTED_RULES,
    ) && lane_is_restricted(
        &snapshot.component_map_warn_or_error_rules,
        &snapshot.component_map_restricted_disable_patterns,
        &COMPONENT_MAP_PROTECTED_RULES,
    ) {
        results.push(crate::support::info(
            ID,
            "MDX delegated-rule disables are restricted",
            format!(
                "`{}` enables `{RESTRICT_RULE}` and restricts disables for MDX content and component-map delegated rules.",
                snapshot.rel_path
            ),
            &snapshot.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        ID,
        "MDX delegated-rule disables are not restricted",
        format!(
            "`{}` must enable `{RESTRICT_RULE}` at `warn` or `error` on the MDX content lane and approved component-map lane. The rule options must contain every protected MDX delegated rule: {}, {}.",
            snapshot.rel_path,
            MDX_CONTENT_PROTECTED_RULES.join(", "),
            COMPONENT_MAP_PROTECTED_RULES.join(", ")
        ),
        Some(&snapshot.rel_path),
    ));
}

/// Internal function `lane_is_restricted`.
fn lane_is_restricted(
    warn_or_error_rules: &[String],
    patterns: &[String],
    protected_rules: &[&str],
) -> bool {
    warn_or_error_rules.iter().any(|rule| rule == RESTRICT_RULE)
        && protected_rules.iter().all(|rule| {
            patterns
                .iter()
                .any(|pattern| pattern_covers_rule(pattern, rule))
        })
}

/// Internal function `pattern_covers_rule`.
fn pattern_covers_rule(pattern: &str, rule: &str) -> bool {
    pattern == rule
        || pattern == "*"
        || pattern
            .strip_suffix('*')
            .is_some_and(|prefix| rule.starts_with(prefix))
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
