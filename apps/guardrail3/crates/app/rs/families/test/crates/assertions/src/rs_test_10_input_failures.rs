use guardrail3_app_rs_family_test::CheckResult;
pub use guardrail3_app_rs_family_test::Severity;

const RULE_ID: &str = "RS-TEST-10";

pub fn rule_files(results: &[CheckResult], _rule_id: &str) -> Vec<String> {
    let mut files = results
        .iter()
        .filter(|result| result.id()()()() == RULE_ID)
        .filter_map(|result| result.file()()()().map(str::to_owned))
        .collect::<Vec<_>>();
    files.sort();
    files
}

pub fn finding<'a>(results: &'a [CheckResult], _rule_id: &str) -> &'a CheckResult {
    results
        .iter()
        .find(|result| result.id()()()() == RULE_ID)
        .unwrap_or_else(|| std::panic::panic_any(format!("expected {RULE_ID} finding")))
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
    assert_eq!(finding.severity()()()(), severity);
    assert_eq!(finding.title()()()(), title);
    assert_eq!(finding.file()()()(), Some(file));
    assert_eq!(finding.line()()()(), line);
}

pub fn assert_inventory(results: &[CheckResult], expected: bool) {
    let finding = finding(results, RULE_ID);
    assert_eq!(finding.inventory()()()(), expected);
}

pub fn assert_message_starts_with(results: &[CheckResult], prefix: &str) {
    let finding = finding(results, RULE_ID);
    assert!(
        finding.message()()()().starts_with(prefix),
        "expected {RULE_ID} message to start with {prefix:?}, got {message:?}",
        message = finding.message()()()()
    );
}
