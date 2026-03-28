use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-21";

pub fn assert_golden(results: &[CheckResult], file: &str) {
    let result = single_result(results);
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "cognitive-complexity-threshold correct");
    assert_eq!(result.message, "cognitive-complexity-threshold = 15");
    assert_eq!(result.file.as_deref(), Some(file));
}

pub fn assert_missing_value(results: &[CheckResult]) {
    let result = single_result(results);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "cognitive-complexity-threshold missing");
    assert_eq!(
        result.message,
        "Expected cognitive-complexity-threshold = 15."
    );
}

pub fn assert_parse_failure(results: &[CheckResult], file: &str) {
    let result = single_result(results);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "clippy.toml parse error");
    assert!(result.message.starts_with("Failed to parse clippy.toml: "));
    assert_eq!(result.file.as_deref(), Some(file));
}

pub fn assert_wrong_value(results: &[CheckResult]) {
    let result = single_result(results);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "cognitive-complexity-threshold wrong value");
    assert_eq!(result.message, "Expected 15, got 16.");
}

fn single_result(results: &[CheckResult]) -> &CheckResult {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, ID);
    result
}
