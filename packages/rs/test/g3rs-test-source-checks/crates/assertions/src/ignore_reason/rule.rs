pub use g3rs_test_ingestion_runtime::fixtures::{file, input};
use g3rs_test_types::G3RsTestSourceChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn check(input: &G3RsTestSourceChecksInput) -> Vec<G3CheckResult> {
    g3rs_test_source_checks_runtime::check(input)
}

pub fn assert_has_result(
    results: &[G3CheckResult],
    rule_id: &str,
    severity: G3Severity,
    title: &str,
    file: &str,
    line: Option<usize>,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == rule_id
                && result.severity() == severity
                && result.title() == title
                && result.file() == Some(file)
                && result.line() == line
        }),
        "missing {rule_id} result: severity={severity:?} title={title:?} file={file:?} line={line:?}\nactual={results:#?}"
    );
}

pub fn assert_has_inventory(results: &[G3CheckResult], rule_id: &str, title: &str, file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == rule_id
                && result.title() == title
                && result.file() == Some(file)
                && result.inventory()
        }),
        "missing inventory {rule_id} result: title={title:?} file={file:?}\nactual={results:#?}"
    );
}

pub fn assert_title_count(results: &[G3CheckResult], rule_id: &str, title: &str, count: usize) {
    assert_eq!(
        results
            .iter()
            .filter(|result| result.id() == rule_id && result.title() == title)
            .count(),
        count,
        "{results:#?}"
    );
}

pub fn assert_message_count(
    results: &[G3CheckResult],
    rule_id: &str,
    title: &str,
    message: &str,
    count: usize,
) {
    assert_eq!(
        results
            .iter()
            .filter(|result| {
                result.id() == rule_id && result.title() == title && result.message() == message
            })
            .count(),
        count,
        "{results:#?}"
    );
}
