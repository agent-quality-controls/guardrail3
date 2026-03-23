use crate::domain::report::Severity;

use super::check;
use super::super::test_support::StubToolChecker;

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
