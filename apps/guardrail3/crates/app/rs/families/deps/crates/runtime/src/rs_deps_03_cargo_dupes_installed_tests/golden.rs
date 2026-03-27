use super::{ExpectedRuleResult, assert_rule_results, tool_facts, tool_input};
use guardrail3_domain_report::Severity;

#[test]
fn inventories_installed_cargo_dupes() {
    let facts = tool_facts("cargo-dupes", true);
    let input = tool_input(&facts, "cargo-dupes");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            title: Some("cargo-dupes installed"),
            inventory: Some(true),
            ..ExpectedRuleResult::default()
        }],
    );
}
