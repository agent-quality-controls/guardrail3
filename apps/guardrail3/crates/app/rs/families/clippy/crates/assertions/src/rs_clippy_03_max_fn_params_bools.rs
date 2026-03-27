use guardrail3_domain_report::{CheckResult, Severity};

use crate::common::{assert_correct, assert_missing, assert_parse_error, single_result};

const ID: &str = "RS-CLIPPY-03";

pub fn assert_golden(results: &[CheckResult], file: &str) {
    assert_correct(
        results,
        ID,
        "max-fn-params-bools correct",
        "max-fn-params-bools = 3",
        file,
    );
}

pub fn assert_missing_value(results: &[CheckResult]) {
    assert_missing(
        results,
        ID,
        "max-fn-params-bools missing",
        "Expected max-fn-params-bools = 3.",
    );
}

pub fn assert_parse_failure(results: &[CheckResult], file: &str) {
    assert_parse_error(results, ID, file);
}

pub fn assert_wrong_value(results: &[CheckResult]) {
    let result = single_result(results, ID);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "max-fn-params-bools wrong value");
    assert_eq!(result.message, "Expected 3, got 4.");
}
