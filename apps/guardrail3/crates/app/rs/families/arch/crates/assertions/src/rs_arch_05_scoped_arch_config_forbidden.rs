use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

use crate::common;

const RULE_ID: &str = "RS-ARCH-05";

pub fn check_results(tree: &ProjectTree) -> Vec<CheckResult> {
    common::check_results(tree)
}

pub fn error_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    common::error_results(results, if rule_id.is_empty() { RULE_ID } else { rule_id })
}

pub fn assert_error_files(results: &[CheckResult], rule_id: &str, expected: &[&str]) {
    common::assert_error_files(
        results,
        if rule_id.is_empty() { RULE_ID } else { rule_id },
        expected,
    );
}
