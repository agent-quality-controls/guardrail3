use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_09_ban_baseline_complete as assertions;

use crate::inputs::ConfigDenyInput;
use super::super::check;
use super::helpers::{build_fixture_deny_toml, config_facts_with_profile, remove_deny_ban};

#[test]
fn emits_no_result_for_generated_library_ban_baseline() {
    let results =
        super::helpers::run_check_with_profile(&build_fixture_deny_toml("library"), "library");

    assert!(
        results.is_empty(),
        "expected canonical library deny baseline to pass: {results:#?}"
    );
}

#[test]
fn library_profile_requires_library_io_bans() {
    let config = config_facts_with_profile(
        &remove_deny_ban(
            &remove_deny_ban(&build_fixture_deny_toml("library"), "axum"),
            "tokio",
        ),
        "library",
    );
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

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
