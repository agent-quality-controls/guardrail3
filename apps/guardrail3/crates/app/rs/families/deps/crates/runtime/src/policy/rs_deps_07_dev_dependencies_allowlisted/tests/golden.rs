use super::{dependency_facts, dependency_input};
use guardrail3_app_rs_family_deps_assertions::rs_deps_07_dev_dependencies_allowlisted as assertions;

#[test]
fn inventories_allowlisted_dev_dependency() {
    let facts = dependency_facts(true, true, "insta");
    let input = dependency_input(&facts, "crates/api/Cargo.toml", "insta");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("dev dependency allowlisted"),
            inventory: Some(true),
            ..assertions::ExpectedRuleResult::default()
        }],
    );
}
