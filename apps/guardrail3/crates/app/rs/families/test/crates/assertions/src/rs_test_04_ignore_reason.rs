use super::expected_finding;
use guardrail3_app_rs_family_test::CheckResult;
pub use guardrail3_app_rs_family_test::Severity;

const RULE_ID: &str = "RS-TEST-04";

pub fn rule_files(results: &[CheckResult], _rule_id: &str) -> Vec<String> {
    let mut files = results
        .iter()
        .filter(|result| result.id == RULE_ID)
        .filter_map(|result| result.file.clone())
        .collect::<Vec<_>>();
    files.sort();
    files
}

pub fn finding<'a>(results: &'a [CheckResult], _rule_id: &str) -> &'a CheckResult {
    expected_finding(results, RULE_ID)
}

pub fn assert_rule_files(results: &[CheckResult], expected: Vec<String>) {
    assert_eq!(
        rule_files(results, RULE_ID),
        expected,
        "unexpected {RULE_ID} files"
    );
}

pub fn assert_rule_quiet(results: &[CheckResult]) {
    assert!(
        rule_files(results, RULE_ID).is_empty(),
        "expected no {RULE_ID} findings"
    );
}

pub fn assert_reported(
    results: &[CheckResult],
    file: &str,
    line: Option<usize>,
    severity: Severity,
    title: &str,
) {
    let finding = finding(results, RULE_ID);
    assert_eq!(finding.severity, severity);
    assert_eq!(finding.title, title);
    assert_eq!(finding.file.as_deref(), Some(file));
    assert_eq!(finding.line, line);
}

pub fn assert_inventory(results: &[CheckResult], expected: bool) {
    let finding = finding(results, RULE_ID);
    assert_eq!(finding.inventory, expected);
}
