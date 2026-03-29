use super::{dependency_facts, dependency_input};
use guardrail3_app_rs_family_deps_assertions::rs_deps_06_build_dependencies_allowlisted as assertions;
use guardrail3_domain_report::Severity;

#[test]
fn inventories_allowlisted_build_dependency() {
    let facts = dependency_facts(true, true, "cc");
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        "cc",
    );
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(Severity::Info),
            title: Some("build dependency allowlisted"),
            inventory: Some(true),
            ..assertions::ExpectedRuleResult::default()
        }],
    );
}
