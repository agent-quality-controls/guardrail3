use g3ts_astro_setup_types::{
    G3TsAstroSetupEslintPluginContractInput, G3TsAstroSetupEslintSurfaceState,
};
use guardrail3_check_types::G3CheckResult;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3ts-astro-setup/unused-eslint-disables-fail";

/// Validates the rule and pushes findings into `results`.
pub(crate) fn check(
    contract: &G3TsAstroSetupEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } = &contract.config else {
        results.push(crate::support::error(
            ID,
            "Unused ESLint disable policy cannot be checked",
            "G3TS could not parse the Astro app ESLint effective config. Configure unused disable reporting to fail at error severity on Astro, TS, and TSX source lanes.".to_owned(),
            Some(config_rel_path(&contract.config)),
        ));
        return;
    };

    if snapshot.astro_source_unused_disable_fail_closed
        && snapshot.ts_source_unused_disable_fail_closed
        && snapshot.tsx_source_unused_disable_fail_closed
    {
        results.push(crate::support::info(
            ID,
            "Unused ESLint disables fail closed on Astro source lanes",
            format!(
                "`{}` makes unused `eslint-disable` directives fail at error severity on Astro, TS, and TSX source probes.",
                snapshot.rel_path
            ),
            &snapshot.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        ID,
        "Unused ESLint disables do not fail closed on every Astro source lane",
        format!(
            "`{}` must either enable `@eslint-community/eslint-comments/no-unused-disable` at `error` or configure ESLint core unused-disable reporting at `error` on Astro, TS, and TSX source lanes. Stale disables otherwise hide bypasses after the original violation disappears.",
            snapshot.rel_path
        ),
        Some(&snapshot.rel_path),
    ));
}

/// Internal helper used by the rule.
fn config_rel_path(config: &G3TsAstroSetupEslintSurfaceState) -> &str {
    match config {
        G3TsAstroSetupEslintSurfaceState::Missing { rel_path }
        | G3TsAstroSetupEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroSetupEslintSurfaceState::ParseError { rel_path, .. } => rel_path,
        G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } => &snapshot.rel_path,
    }
}
