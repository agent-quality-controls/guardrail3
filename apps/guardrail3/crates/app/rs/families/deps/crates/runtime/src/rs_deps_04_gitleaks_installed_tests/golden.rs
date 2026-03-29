use super::{tool_facts, tool_input};
use guardrail3_app_rs_family_deps_assertions::rs_deps_04_gitleaks_installed as assertions;

#[test]
fn inventories_installed_gitleaks() {
    let facts = tool_facts("gitleaks", true);
    let input = tool_input(&facts, "gitleaks");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("gitleaks installed"),
            inventory: Some(true),
            ..assertions::ExpectedRuleResult::default()
        }],
    );
}
