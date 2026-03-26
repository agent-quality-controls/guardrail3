use std::path::Path;

use guardrail3_app_rs_family_hexarch as runtime;
use guardrail3_domain_report::{CheckResult, Severity};

const RULE_ID: &str = "RS-HEXARCH-25";

pub fn run_family(root: &Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    runtime::check_test_tree(&tree)
}

pub fn check_results(root: &Path) -> Vec<CheckResult> {
    run_family(root)
}

pub fn error_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    results
        .iter()
        .filter(|result| result.id == rule_id && result.severity == Severity::Error)
        .collect()
}

pub fn errors_by_id<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    error_results(results, rule_id)
}

pub fn assert_no_error(results: &[CheckResult], rule_id: &str) {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    let errors = error_results(results, rule_id);
    assert!(
        errors.is_empty(),
        "expected no {rule_id} errors, got: {errors:#?}"
    );
}

