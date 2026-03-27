use guardrail3_domain_report::CheckResult;

pub use super::common::ExpectedRuleResult;

const RULE_ID: &str = "RS-DEPS-10";

pub fn rule_results<'a>(results: &'a [CheckResult]) -> Vec<&'a CheckResult> {
    super::common::rule_results(results, RULE_ID)
}

pub fn assert_rule_results(results: &[CheckResult], expected: &[ExpectedRuleResult<'_>]) {
    super::common::assert_rule_results(results, RULE_ID, expected);
}
