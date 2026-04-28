use g3ts_astro_setup_types::{
    G3TsAstroSetupEslintPluginContractInput, G3TsAstroSetupEslintSurfaceState,
};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-setup/protected-setup-rule-disables-restricted";
const RESTRICT_RULE: &str = "@eslint-community/eslint-comments/no-restricted-disable";
const ASTRO_SOURCE_PROTECTED_RULES: [&str; 2] =
    ["astro/valid-compile", "@eslint-community/eslint-comments/*"];
const TS_SOURCE_PROTECTED_RULES: [&str; 1] = ["@eslint-community/eslint-comments/*"];

pub(crate) fn check(
    contract: &G3TsAstroSetupEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } = &contract.config else {
        results.push(crate::support::error(
            ID,
            "Setup protected-disable policy cannot be checked",
            "G3TS could not parse the Astro setup ESLint effective config. Configure `@eslint-community/eslint-comments/no-restricted-disable` on Astro, TS, and TSX source lanes for setup-owned delegated rules.".to_owned(),
            config_rel_path(&contract.config),
        ));
        return;
    };

    if lane_is_restricted(
        &snapshot.astro_source_warn_or_error_rules,
        &snapshot.astro_source_restricted_disable_patterns,
        &ASTRO_SOURCE_PROTECTED_RULES,
    ) && lane_is_restricted(
        &snapshot.ts_source_warn_or_error_rules,
        &snapshot.ts_source_restricted_disable_patterns,
        &TS_SOURCE_PROTECTED_RULES,
    ) && lane_is_restricted(
        &snapshot.tsx_source_warn_or_error_rules,
        &snapshot.tsx_source_restricted_disable_patterns,
        &TS_SOURCE_PROTECTED_RULES,
    ) {
        results.push(crate::support::info(
            ID,
            "Astro setup delegated-rule disables are restricted",
            format!("`{}` restricts disables for Astro compile and eslint-comments setup rules on Astro, TS, and TSX source probes.", snapshot.rel_path),
            &snapshot.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        ID,
        "Astro setup delegated-rule disables are not restricted",
        format!(
            "`{}` must enable `{RESTRICT_RULE}` at `warn` or `error` on Astro, TS, and TSX source lanes. Astro source must protect `astro/valid-compile` and all `@eslint-community/eslint-comments/*` rules; TS and TSX source must protect all `@eslint-community/eslint-comments/*` rules.",
            snapshot.rel_path
        ),
        Some(&snapshot.rel_path),
    ));
}

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

fn pattern_covers_rule(pattern: &str, rule: &str) -> bool {
    pattern == rule
        || pattern == "*"
        || pattern
            .strip_suffix('*')
            .is_some_and(|prefix| rule.starts_with(prefix))
}

fn config_rel_path(config: &G3TsAstroSetupEslintSurfaceState) -> Option<&str> {
    match config {
        G3TsAstroSetupEslintSurfaceState::Missing { rel_path }
        | G3TsAstroSetupEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroSetupEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}
