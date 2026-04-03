use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-22";

pub fn assert_golden(results: &[CheckResult], file: &str) {
    let result = single_result(results);
    assert!(result.inventory());
    assert_eq!(result.severity(), Severity::Info);
    assert_eq!(result.title(), "type-complexity-threshold correct");
    assert_eq!(result.message(), "type-complexity-threshold = 75");
    assert_eq!(result.file(), Some(file));
}

pub fn assert_missing_value(results: &[CheckResult]) {
    let result = single_result(results);
    assert_eq!(result.severity(), Severity::Error);
    assert_eq!(result.title(), "type-complexity-threshold missing");
    assert_eq!(result.message(), "Add `type-complexity-threshold = 75` to clippy.toml.");
}

pub fn assert_parse_failure(results: &[CheckResult], file: &str) {
    assert!(
        results.is_empty(),
        "expected RS-CLIPPY-25 to own parse failure for {file}: {results:#?}"
    );
}

pub fn assert_wrong_value(results: &[CheckResult]) {
    let result = single_result(results);
    assert_eq!(result.severity(), Severity::Error);
    assert_eq!(result.title(), "type-complexity-threshold wrong value");
    assert_eq!(result.message(), "Expected 75, got 76. Set `type-complexity-threshold = 75` in clippy.toml.");
}

fn single_result(results: &[CheckResult]) -> &CheckResult {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id(), ID);
    result
}
