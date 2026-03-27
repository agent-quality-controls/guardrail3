use guardrail3_domain_report::{CheckResult, Severity};

pub(crate) fn single_result<'a>(results: &'a [CheckResult], id: &str) -> &'a CheckResult {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, id);
    result
}

pub(crate) fn assert_correct(
    results: &[CheckResult],
    id: &str,
    title: &str,
    message: &str,
    file: &str,
) {
    let result = single_result(results, id);
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, title);
    assert_eq!(result.message, message);
    assert_eq!(result.file.as_deref(), Some(file));
}

pub(crate) fn assert_missing(results: &[CheckResult], id: &str, title: &str, message: &str) {
    let result = single_result(results, id);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, title);
    assert_eq!(result.message, message);
}

pub(crate) fn assert_parse_error(results: &[CheckResult], id: &str, file: &str) {
    let result = single_result(results, id);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "clippy.toml parse error");
    assert!(result.message.starts_with("Failed to parse clippy.toml: "));
    assert_eq!(result.file.as_deref(), Some(file));
}
