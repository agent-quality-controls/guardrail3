use g3rs_deny_config_checks_assertions::ban_baseline_complete as assertions;

use test_support::run;

use super::helpers;

#[test]
fn stays_quiet_for_canonical_library_bans() {
    let results = run(
        &helpers::library_canonical_bans_toml(),
        Some(guardrail3_rs_toml_parser::types::RustProfile::Library),
        true,
        crate::ban_baseline_complete::check,
    );

    assertions::assert_no_findings(&results);
}

#[test]
fn errors_when_canonical_library_ban_is_missing() {
    let deny_toml = helpers::library_canonical_bans_toml()
        .replace("\"axum\",\n", "")
        .replace("\"tokio\",\n", "");
    let results = run(
        &deny_toml,
        Some(guardrail3_rs_toml_parser::types::RustProfile::Library),
        true,
        crate::ban_baseline_complete::check,
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "missing canonical ban",
                "`deny.toml` is missing deny ban `axum`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "missing canonical ban",
                "`deny.toml` is missing deny ban `tokio`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
