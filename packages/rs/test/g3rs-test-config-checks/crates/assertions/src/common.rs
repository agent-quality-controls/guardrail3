use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_has_result(
    results: &[G3CheckResult],
    id: &str,
    severity: G3Severity,
    title: &str,
    file: &str,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == id
                && result.severity() == severity
                && result.title() == title
                && result.file() == Some(file)
        }),
        "missing result {id} {severity:?} {title} {file}; results: {results:#?}"
    );
}

pub fn assert_quiet(results: &[G3CheckResult]) {
    assert!(results.is_empty(), "expected no findings; got: {results:#?}");
}
