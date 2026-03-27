use guardrail3_domain_report::{CheckResult, Severity};

use crate::common::{assert_correct, assert_missing, assert_parse_error, single_result};

const ID: &str = "RS-CLIPPY-21";

pub fn assert_golden(results: &[CheckResult], file: &str) {
    assert_correct(
        results,
        ID,
        "cognitive-complexity-threshold correct",
        "cognitive-complexity-threshold = 15",
        file,
    );
}

pub fn assert_missing_value(results: &[CheckResult]) {
    assert_missing(
        results,
        ID,
        "cognitive-complexity-threshold missing",
        "Expected cognitive-complexity-threshold = 15.",
    );
}

pub fn assert_parse_failure(results: &[CheckResult], file: &str) {
    assert_parse_error(results, ID, file);
}

pub fn assert_wrong_value(results: &[CheckResult]) {
    let result = single_result(results, ID);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "cognitive-complexity-threshold wrong value");
    assert_eq!(result.message, "Expected 15, got 16.");
}
