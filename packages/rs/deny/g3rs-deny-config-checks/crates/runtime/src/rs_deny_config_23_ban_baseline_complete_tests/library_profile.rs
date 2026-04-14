use g3rs_deny_config_checks_assertions::rs_deny_config_23_ban_baseline_complete as assertions;

use crate::test_support::{canonical_bans_toml, run};

#[test]
fn stays_quiet_for_canonical_library_bans() {
    let results = run(
        &canonical_bans_toml("library"),
        Some("library"),
        true,
        crate::rs_deny_config_23_ban_baseline_complete::check,
    );

    assertions::assert_no_findings(&results);
}

#[test]
fn errors_when_canonical_library_ban_is_missing() {
    let deny_toml = canonical_bans_toml("library")
        .replace("\"axum\",\n", "")
        .replace("\"tokio\",\n", "");
    let results = run(
        &deny_toml,
        Some("library"),
        true,
        crate::rs_deny_config_23_ban_baseline_complete::check,
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
