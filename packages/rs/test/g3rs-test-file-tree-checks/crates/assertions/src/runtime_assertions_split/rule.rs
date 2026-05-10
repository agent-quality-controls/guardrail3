#![expect(
    clippy::expect_used,
    reason = "structural assertion helper code where lint conflicts with verification surface"
)]
#[must_use]
pub fn check(
    input: &g3rs_test_types::G3RsTestFileTreeChecksInput,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    g3rs_test_file_tree_checks_runtime::check(input)
}

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
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

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
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

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_message(
    results: &[guardrail3_check_types::G3CheckResult],
    rule_id: &str,
    title: &str,
    file: &str,
    message: &str,
) {
    let result = find_result(results, rule_id, title, file).expect("missing expected result");
    assert_eq!(result.message(), message, "assertion failed");
}

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_no_title(
    results: &[guardrail3_check_types::G3CheckResult],
    rule_id: &str,
    title: &str,
) {
    assert!(
        results
            .iter()
            .all(|result| !(result.id() == rule_id && result.title() == title)),
        "unexpected {rule_id} result with title={title:?}\nactual={results:#?}"
    );
}
