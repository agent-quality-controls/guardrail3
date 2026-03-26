use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

const RULE_ID: &str = "RS-CARGO-06";

pub fn check_results(tree: &ProjectTree) -> Vec<CheckResult> {
    crate::common::check_results(tree)
}

pub fn rule_results<'a>(results: &'a [CheckResult], _rule_id: &str) -> Vec<&'a CheckResult> {
    crate::common::rule_results(results, RULE_ID)
}

pub fn assert_result_count(results: &[CheckResult], expected: usize) {
    assert_eq!(
        rule_results(results, RULE_ID).len(),
        expected,
        "unexpected {RULE_ID} results: {results:#?}"
    );
}
