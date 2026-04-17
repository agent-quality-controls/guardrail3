use g3rs_hooks_config_checks_assertions::hook_rs_14_guardrail_binary_available::rule as assertions;

use super::helpers::hook;
use super::super::check;

#[test]
fn stays_quiet_when_hook_does_not_require_g3rs() {
    let mut results = Vec::new();
    check(
        &hook("cargo fmt --check\n"),
        &[],
        &mut results,
    );

    assertions::assert_no_findings(&results);
}

#[test]
fn reports_inventory_when_g3rs_is_installed() {
    let mut results = Vec::new();
    check(
        &hook("g3rs validate --path .\n"),
        &["g3rs".to_owned()],
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::inventory(
            "g3rs binary available",
            "g3rs is available for fail-closed Rust hook validation.",
            ".githooks/pre-commit",
        ),
    );
}

#[test]
fn reports_inventory_when_g3rs_is_path_qualified() {
    let mut results = Vec::new();
    check(
        &hook("/usr/local/bin/g3rs validate --path .\n"),
        &[],
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::inventory(
            "g3rs binary available",
            "g3rs is available for fail-closed Rust hook validation.",
            ".githooks/pre-commit",
        ),
    );
}

#[test]
fn reports_missing_g3rs_when_validation_is_required() {
    let mut results = Vec::new();
    check(
        &hook("g3rs validate --path .\n"),
        &[],
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::error(
            "g3rs binary missing",
            "Hook requires g3rs, but it is not available on PATH.",
            ".githooks/pre-commit",
        ),
    );
}
