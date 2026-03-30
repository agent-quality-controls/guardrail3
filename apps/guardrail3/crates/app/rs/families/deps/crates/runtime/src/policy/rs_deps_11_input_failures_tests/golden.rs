use super::{failure_facts, failure_input};
use guardrail3_app_rs_family_deps_assertions::rs_deps_11_input_failures as assertions;

#[test]
fn emits_error_for_input_failure() {
    let facts = failure_facts("guardrail3.toml", "parse failed");
    let input = failure_input(&facts, "guardrail3.toml");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            message: Some("parse failed"),
            ..assertions::ExpectedRuleResult::default()
        }],
    );
}
