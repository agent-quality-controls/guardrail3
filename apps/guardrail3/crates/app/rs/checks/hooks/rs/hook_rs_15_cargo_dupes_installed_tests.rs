use crate::domain::report::Severity;

use super::super::test_support::StubToolChecker;
use super::check;

#[test]
fn passes_when_cargo_dupes_is_installed() {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        true,
        false,
        &StubToolChecker::new(&["cargo-dupes"]),
        &mut results,
    );
    assert_eq!(results.len(), 1);
    assert!(results[0].inventory);
}

#[test]
fn errors_when_cargo_dupes_is_missing() {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        true,
        false,
        &StubToolChecker::new(&[]),
        &mut results,
    );
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "cargo-dupes missing");
    assert!(!results[0].inventory);
}

#[test]
fn skips_when_cargo_dupes_is_not_required() {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        false,
        false,
        &StubToolChecker::new(&[]),
        &mut results,
    );
    assert!(results.is_empty());
}

#[test]
fn passes_when_cargo_dupes_uses_explicit_path() {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        true,
        true,
        &StubToolChecker::new(&[]),
        &mut results,
    );
    assert_eq!(results.len(), 1);
    assert!(results[0].inventory);
}
