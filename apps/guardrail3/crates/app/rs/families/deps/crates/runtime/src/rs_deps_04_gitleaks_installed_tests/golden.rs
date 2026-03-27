use super::{ExpectedRuleResult, assert_rule_results, tool_facts, tool_input};
use guardrail3_domain_report::Severity;

#[test]
fn inventories_installed_gitleaks() {
    let facts = tool_facts("gitleaks", true);
    let input = tool_input(&facts, "gitleaks");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            title: Some("gitleaks installed"),
            inventory: Some(true),
            ..ExpectedRuleResult::default()
        }],
    );
}
