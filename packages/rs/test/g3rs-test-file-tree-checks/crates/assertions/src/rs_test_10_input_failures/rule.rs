pub fn check(
    input: &g3rs_test_types::G3RsTestFileTreeChecksInput,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    g3rs_test_file_tree_checks_runtime::check(input)
}

pub fn assert_has_result(
    results: &[guardrail3_check_types::G3CheckResult],
    rule_id: &str,
    severity: guardrail3_check_types::G3Severity,
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

pub fn assert_no_rule(results: &[guardrail3_check_types::G3CheckResult], rule_id: &str) {
    assert!(
        results.iter().all(|result| result.id() != rule_id),
        "unexpected {rule_id} result present\nactual={results:#?}"
    );
}

#[must_use]
pub fn find_result<'a>(
    results: &'a [guardrail3_check_types::G3CheckResult],
    rule_id: &str,
    title: &str,
    file: &str,
) -> Option<&'a guardrail3_check_types::G3CheckResult> {
    results.iter().find(|result| {
        result.id() == rule_id && result.title() == title && result.file() == Some(file)
    })
}

pub fn assert_message(
    results: &[guardrail3_check_types::G3CheckResult],
    rule_id: &str,
    title: &str,
    file: &str,
    message: &str,
) {
    let result = find_result(results, rule_id, title, file).expect("missing expected result");
    assert_eq!(result.message(), message);
}
