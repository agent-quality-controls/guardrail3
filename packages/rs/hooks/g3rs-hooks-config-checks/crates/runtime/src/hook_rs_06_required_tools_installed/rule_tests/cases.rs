use g3rs_hooks_config_checks_assertions::hook_rs_06_required_tools_installed::rule as assertions;

use super::helpers::selected_hook;
use super::super::check;

#[test]
fn reports_required_tools_as_inventory_when_installed() {
    let mut results = Vec::new();
    check(
        &selected_hook("gitleaks protect --staged --no-banner\ncargo-deny check\ncargo-machete\n"),
        &[
            "gitleaks".to_owned(),
            "cargo-deny".to_owned(),
            "cargo-machete".to_owned(),
        ],
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::inventory(
            "gitleaks installed",
            "gitleaks is available for Rust hook execution.",
            ".githooks/pre-commit",
        ),
    );
    assertions::assert_contains(
        &results,
        assertions::inventory(
            "cargo-deny installed",
            "cargo-deny is available for Rust hook execution.",
            ".githooks/pre-commit",
        ),
    );
    assertions::assert_contains(
        &results,
        assertions::inventory(
            "cargo-machete installed",
            "cargo-machete is available for Rust hook execution.",
            ".githooks/pre-commit",
        ),
    );
}

#[test]
fn reports_missing_tools_as_errors() {
    let mut results = Vec::new();
    check(
        &selected_hook("gitleaks protect --staged --no-banner\ncargo-deny check\ncargo-machete\n"),
        &["gitleaks".to_owned()],
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::inventory(
            "gitleaks installed",
            "gitleaks is available for Rust hook execution.",
            ".githooks/pre-commit",
        ),
    );
    assertions::assert_contains(
        &results,
        assertions::error(
            "cargo-deny missing",
            "cargo-deny is required by the Rust hook but is not available on PATH or via a path-qualified command.",
            ".githooks/pre-commit",
        ),
    );
    assertions::assert_contains(
        &results,
        assertions::error(
            "cargo-machete missing",
            "cargo-machete is required by the Rust hook but is not available on PATH or via a path-qualified command.",
            ".githooks/pre-commit",
        ),
    );
}

#[test]
fn treats_path_qualified_tools_as_installed() {
    let mut results = Vec::new();
    check(
        &selected_hook(
            "/opt/bin/gitleaks protect --staged --no-banner\n/opt/bin/cargo-deny check\n/opt/bin/cargo-machete\n",
        ),
        &[],
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::inventory(
            "gitleaks installed",
            "gitleaks is available for Rust hook execution.",
            ".githooks/pre-commit",
        ),
    );
    assertions::assert_contains(
        &results,
        assertions::inventory(
            "cargo-deny installed",
            "cargo-deny is available for Rust hook execution.",
            ".githooks/pre-commit",
        ),
    );
    assertions::assert_contains(
        &results,
        assertions::inventory(
            "cargo-machete installed",
            "cargo-machete is available for Rust hook execution.",
            ".githooks/pre-commit",
        ),
    );
}
