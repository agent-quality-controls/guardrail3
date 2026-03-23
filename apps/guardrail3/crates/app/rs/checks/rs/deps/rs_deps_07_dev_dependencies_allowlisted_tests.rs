use crate::domain::report::Severity;

use super::super::facts::DependencySectionKind;
use super::super::test_support::{dependency_facts, dependency_input};
use super::check;

#[test]
fn inventories_allowlisted_dev_dependency() {
    let facts = dependency_facts(DependencySectionKind::DevDependencies, true, true, "insta");
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        DependencySectionKind::DevDependencies,
        "insta",
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn warns_on_unauthorized_dev_dependency() {
    let facts = dependency_facts(
        DependencySectionKind::DevDependencies,
        true,
        false,
        "tempfile",
    );
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        DependencySectionKind::DevDependencies,
        "tempfile",
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(
        results[0].message,
        "Dependency `tempfile` in `[dev-dependencies]` is not allowlisted for crate `api`."
    );
}

#[test]
fn no_allowlist_means_dev_dependency_rule_stays_silent() {
    let facts = dependency_facts(
        DependencySectionKind::DevDependencies,
        false,
        false,
        "tempfile",
    );
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        DependencySectionKind::DevDependencies,
        "tempfile",
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}
