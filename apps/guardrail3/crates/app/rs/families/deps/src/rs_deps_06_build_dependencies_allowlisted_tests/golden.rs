use crate::facts::DependencySectionKind;
use crate::test_support::{dependency_facts, dependency_input};
use guardrail3_domain_report::Severity;

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

    super::super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DEPS-06");
    assert_eq!(result.severity, Severity::Info);
    assert!(result.inventory);
    assert_eq!(result.title, "build dependency allowlisted");
}
