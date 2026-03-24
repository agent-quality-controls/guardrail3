use crate::app::rs::checks::rs::deps::facts::DependencySectionKind;
use crate::app::rs::checks::rs::deps::test_support::{dependency_facts, dependency_input};
use crate::domain::report::Severity;

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

    super::super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DEPS-07");
    assert_eq!(result.severity, Severity::Info);
    assert!(result.inventory);
    assert_eq!(result.title, "dev dependency allowlisted");
}
