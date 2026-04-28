use g3ts_astro_content_types::{
    G3TsAstroContentEslintPluginContractInput, G3TsAstroContentEslintSurfaceState,
};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-content/protected-content-rule-disables-restricted";
const RESTRICT_RULE: &str = "@eslint-community/eslint-comments/no-restricted-disable";
const PROTECTED_RULES: [&str; 9] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
    "astro-pipeline/no-content-data-modules-in-routes",
    "astro-pipeline/no-direct-astro-content-in-routes",
    "astro-pipeline/require-approved-content-adapter-in-routes",
    "astro-pipeline/no-side-loader-imports",
    "astro-pipeline/no-velite-imports",
    "i18next/no-literal-string",
];

pub(crate) fn check(
    contract: &G3TsAstroContentEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = &contract.config else {
        results.push(crate::support::error(
            ID,
            "Content protected-disable policy cannot be checked",
            "G3TS could not parse the Astro content ESLint effective config. Configure `@eslint-community/eslint-comments/no-restricted-disable` on Astro, TS, and TSX source lanes for all protected content rules.".to_owned(),
            config_rel_path(&contract.config),
        ));
        return;
    };

    if lane_is_restricted(
        &snapshot.astro_source_warn_or_error_rules,
        &snapshot.astro_source_restricted_disable_patterns,
    ) && lane_is_restricted(
        &snapshot.ts_source_warn_or_error_rules,
        &snapshot.ts_source_restricted_disable_patterns,
    ) && lane_is_restricted(
        &snapshot.tsx_source_warn_or_error_rules,
        &snapshot.tsx_source_restricted_disable_patterns,
    ) {
        results.push(crate::support::info(
            ID,
            "Content delegated-rule disables are restricted",
            format!(
                "`{}` enables `{RESTRICT_RULE}` and restricts disables for Astro content pipeline and inline-copy rules on Astro, TS, and TSX source probes.",
                snapshot.rel_path
            ),
            &snapshot.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        ID,
        "Content delegated-rule disables are not restricted",
        format!(
            "`{}` must enable `{RESTRICT_RULE}` at `warn` or `error` on Astro, TS, and TSX source lanes, with options containing every protected rule: {}. This keeps agents from bypassing Astro content architecture with `eslint-disable` comments.",
            snapshot.rel_path,
            PROTECTED_RULES.join(", ")
        ),
        Some(&snapshot.rel_path),
    ));
}

fn lane_is_restricted(warn_or_error_rules: &[String], patterns: &[String]) -> bool {
    warn_or_error_rules.iter().any(|rule| rule == RESTRICT_RULE)
        && PROTECTED_RULES.iter().all(|rule| {
            patterns
                .iter()
                .any(|pattern| pattern_covers_rule(pattern, rule))
        })
}

fn pattern_covers_rule(pattern: &str, rule: &str) -> bool {
    pattern == rule
        || pattern == "*"
        || pattern
            .strip_suffix('*')
            .is_some_and(|prefix| rule.starts_with(prefix))
}

fn config_rel_path(config: &G3TsAstroContentEslintSurfaceState) -> Option<&str> {
    match config {
        G3TsAstroContentEslintSurfaceState::Missing { rel_path }
        | G3TsAstroContentEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroContentEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroContentEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}
