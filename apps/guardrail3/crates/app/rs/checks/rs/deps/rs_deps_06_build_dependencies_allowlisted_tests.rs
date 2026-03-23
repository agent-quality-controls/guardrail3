use crate::domain::report::Severity;

use super::super::facts::DependencySectionKind;
use super::super::test_support::{dependency_facts, dependency_input};
use super::check;

#[test]
fn inventories_allowlisted_build_dependency() {
    let facts = dependency_facts(DependencySectionKind::BuildDependencies, true, true, "cc");
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        DependencySectionKind::BuildDependencies,
        "cc",
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn errors_on_unauthorized_build_dependency() {
    let facts = dependency_facts(
        DependencySectionKind::BuildDependencies,
        true,
        false,
        "bindgen",
    );
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        DependencySectionKind::BuildDependencies,
        "bindgen",
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(
        results[0].message,
        "Dependency `bindgen` in `[build-dependencies]` is not allowlisted for crate `api`."
    );
}

#[test]
fn no_allowlist_means_build_dependency_rule_stays_silent() {
    let facts = dependency_facts(
        DependencySectionKind::BuildDependencies,
        false,
        false,
        "bindgen",
    );
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        DependencySectionKind::BuildDependencies,
        "bindgen",
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}
