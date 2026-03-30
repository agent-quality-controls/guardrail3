crate::define_rule_assertions!("RS-DEPS-12");

pub type ExpectedInputFailureResult<'a> = crate::common::ExpectedRuleResult<'a>;
pub use guardrail3_domain_report::Severity as InputFailureSeverity;

pub fn assert_input_failure_results(
    results: &[guardrail3_domain_report::CheckResult],
    expected: &[ExpectedInputFailureResult],
) {
    crate::common::assert_rule_results(results, "RS-DEPS-11", expected);
}
