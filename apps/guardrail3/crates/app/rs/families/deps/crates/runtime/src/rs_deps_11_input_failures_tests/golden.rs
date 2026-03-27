use super::{ExpectedRuleResult, assert_rule_results, failure_facts, failure_input};
use guardrail3_domain_report::Severity;

#[test]
fn emits_error_for_input_failure() {
    let facts = failure_facts("guardrail3.toml", "parse failed");
    let input = failure_input(&facts, "guardrail3.toml");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            message: Some("parse failed"),
            ..ExpectedRuleResult::default()
        }],
    );
}
