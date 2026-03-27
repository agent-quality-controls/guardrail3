use super::{ExpectedRuleResult, assert_rule_results, coverage_facts, coverage_input};
use guardrail3_domain_report::Severity;

#[test]
fn inventories_library_allowlist_when_present() {
    let facts = coverage_facts(Some("library"), true);
    let input = coverage_input(&facts, "packages/core/Cargo.toml");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            title: Some("library allowlist present"),
            inventory: Some(true),
            ..ExpectedRuleResult::default()
        }],
    );
}
