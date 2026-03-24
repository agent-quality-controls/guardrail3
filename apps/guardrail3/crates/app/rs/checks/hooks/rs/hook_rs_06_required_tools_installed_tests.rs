use crate::domain::report::Severity;

use super::super::test_support::StubToolChecker;
use super::check;

#[test]
fn reports_all_required_tools_as_inventory_when_installed() {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        &StubToolChecker::new(&["gitleaks", "cargo-deny", "cargo-machete"]),
        &mut results,
    );
    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|result| result.inventory));
    assert_eq!(
        results
            .iter()
            .map(|result| result.title.as_str())
            .collect::<Vec<_>>(),
        vec![
            "gitleaks installed",
            "cargo-deny installed",
            "cargo-machete installed"
        ]
    );
}

#[test]
fn reports_missing_tool_as_error() {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        &StubToolChecker::new(&["gitleaks", "cargo-deny"]),
        &mut results,
    );
    assert_eq!(results.len(), 3);
    assert!(results.iter().any(|result| {
        result.title == "cargo-machete missing"
            && result.severity == Severity::Error
            && !result.inventory
    }));
}

#[test]
fn reports_gitleaks_missing_as_error() {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        &StubToolChecker::new(&["cargo-deny", "cargo-machete"]),
        &mut results,
    );
    assert!(results.iter().any(|result| {
        result.title == "gitleaks missing"
            && result.severity == Severity::Error
            && !result.inventory
    }));
}

#[test]
fn reports_cargo_deny_missing_as_error() {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        &StubToolChecker::new(&["gitleaks", "cargo-machete"]),
        &mut results,
    );
    assert!(results.iter().any(|result| {
        result.title == "cargo-deny missing"
            && result.severity == Severity::Error
            && !result.inventory
    }));
}

#[test]
fn reports_all_tools_missing_as_distinct_errors() {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        &StubToolChecker::new(&[]),
        &mut results,
    );
    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|result| !result.inventory));
    assert_eq!(
        results
            .iter()
            .map(|result| result.title.as_str())
            .collect::<Vec<_>>(),
        vec![
            "gitleaks missing",
            "cargo-deny missing",
            "cargo-machete missing"
        ]
    );
}

#[test]
fn reports_mixed_installed_and_missing_tools_in_same_run() {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        &StubToolChecker::new(&["gitleaks"]),
        &mut results,
    );
    assert_eq!(results.len(), 3);
    assert!(
        results
            .iter()
            .any(|result| result.title == "gitleaks installed" && result.inventory)
    );
    assert!(results.iter().any(|result| {
        result.title == "cargo-deny missing"
            && result.severity == Severity::Error
            && !result.inventory
    }));
    assert!(results.iter().any(|result| {
        result.title == "cargo-machete missing"
            && result.severity == Severity::Error
            && !result.inventory
    }));
}

#[test]
fn only_reports_the_three_expected_tool_names() {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        &StubToolChecker::new(&[
            "gitleaks",
            "cargo-deny",
            "cargo-machete",
            "guardrail3",
            "cargo-dupes",
        ]),
        &mut results,
    );
    assert_eq!(results.len(), 3);
    assert_eq!(
        results
            .iter()
            .map(|result| result.title.as_str())
            .collect::<Vec<_>>(),
        vec![
            "gitleaks installed",
            "cargo-deny installed",
            "cargo-machete installed"
        ]
    );
}
