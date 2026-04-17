use g3rs_hooks_config_checks_assertions::hook_rs_15_cargo_dupes_installed::rule as assertions;

use super::helpers::hook;
use super::super::check;

#[test]
fn stays_quiet_when_hook_does_not_require_cargo_dupes() {
    let mut results = Vec::new();
    check(&hook("cargo fmt --check\n"), &[], &mut results);

    assertions::assert_no_findings(&results);
}

#[test]
fn reports_inventory_when_cargo_dupes_is_installed() {
    let mut results = Vec::new();
    check(
        &hook("cargo-dupes check --exclude-tests\n"),
        &["cargo-dupes".to_owned()],
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::inventory(
            "cargo-dupes installed",
            "cargo-dupes is available for Rust duplication checks.",
            ".githooks/pre-commit",
        ),
    );
}

#[test]
fn reports_inventory_when_cargo_dupes_is_path_qualified() {
    let mut results = Vec::new();
    check(
        &hook("/usr/local/bin/cargo-dupes check --exclude-tests\n"),
        &[],
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::inventory(
            "cargo-dupes installed",
            "cargo-dupes is available for Rust duplication checks.",
            ".githooks/pre-commit",
        ),
    );
}

#[test]
fn reports_missing_cargo_dupes_when_required() {
    let mut results = Vec::new();
    check(
        &hook("cargo dupes check --exclude-tests\n"),
        &[],
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::error(
            "cargo-dupes missing",
            "Hook requires cargo-dupes, but it is not available on PATH.",
            ".githooks/pre-commit",
        ),
    );
}
