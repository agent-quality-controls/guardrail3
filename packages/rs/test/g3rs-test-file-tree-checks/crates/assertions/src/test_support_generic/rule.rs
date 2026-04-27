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

pub fn assert_has_inventory(
    results: &[guardrail3_check_types::G3CheckResult],
    rule_id: &str,
    title: &str,
    file: &str,
) {
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
