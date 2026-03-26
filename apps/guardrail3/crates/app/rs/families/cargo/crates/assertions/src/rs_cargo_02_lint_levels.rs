use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

const RULE_ID: &str = "RS-CARGO-02";

pub fn check_results(tree: &ProjectTree) -> Vec<CheckResult> {
    crate::common::check_results(tree)
}

pub fn rule_results<'a>(results: &'a [CheckResult], _rule_id: &str) -> Vec<&'a CheckResult> {
    crate::common::rule_results(results, RULE_ID)
}

pub fn has_result<F>(results: &[CheckResult], _rule_id: &str, predicate: F) -> bool
where
    F: Fn(&CheckResult) -> bool,
{
    crate::common::has_result(results, RULE_ID, predicate)
}

pub fn assert_result_count(results: &[CheckResult], expected: usize) {
    assert_eq!(
        rule_results(results, RULE_ID).len(),
        expected,
        "unexpected {RULE_ID} results: {results:#?}"
    );
}
