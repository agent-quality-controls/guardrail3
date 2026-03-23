use crate::domain::report::Severity;

use super::check;
use super::super::test_support::StubToolChecker;

#[test]
fn passes_when_cargo_dupes_is_installed() {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        &StubToolChecker::new(&["cargo-dupes"]),
        &mut results,
    );
    assert_eq!(results.len(), 1);
    assert!(results[0].inventory);
}

#[test]
fn errors_when_cargo_dupes_is_missing() {
    let mut results = Vec::new();
    check(".githooks/pre-commit", &StubToolChecker::new(&[]), &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "cargo-dupes missing");
    assert!(!results[0].inventory);
}
