use super::{ExpectedRuleResult, assert_rule_results, dependency_facts, dependency_input};
use crate::facts::DependencySectionKind;
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

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            title: Some("build dependency allowlisted"),
            inventory: Some(true),
            ..ExpectedRuleResult::default()
        }],
    );
}
