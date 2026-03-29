use guardrail3_domain_report::{CheckResult, Severity};

const RULE_ID: &str = "RS-HEXARCH-27";
const WORKSPACE_MEMBERS_RULE_ID: &str = "RS-HEXARCH-07";

#[derive(Debug)]
pub struct ExpectedRuleResult<'a> {
    pub file: Option<&'a str>,
    pub file_contains: Option<&'a str>,
    pub title_contains: Option<&'a [&'a str]>,
    pub message_contains: Option<&'a [&'a str]>,
}

pub fn assert_expected_rule_results<'a>(
    results: &[CheckResult],
    expected: &[ExpectedRuleResult<'a>],
) {
    let mut actual = error_results(results);
    assert_eq!(
        actual.len(),
        expected.len(),
        "unexpected RS-HEXARCH-27 results: {actual:#?}"
    );

    for expected_result in expected {
        let index = actual.iter().position(|result| {
            expected_result
                .file
                .is_none_or(|file| result.file.as_deref() == Some(file))
                && expected_result.file_contains.is_none_or(|expected_file| {
                    result
                        .file
                        .as_deref()
                        .is_some_and(|actual_file| actual_file.contains(expected_file))
                })
                && expected_result.title_contains.is_none_or(|needles| {
                    needles.iter().all(|needle| result.title.contains(needle))
                })
                && expected_result.message_contains.is_none_or(|needles| {
                    needles.iter().all(|needle| result.message.contains(needle))
                })
        });

        assert!(
            index.is_some(),
            "missing expected RS-HEXARCH-27 result {expected_result:#?}; actual: {actual:#?}"
        );
        if let Some(index) = index {
            let _ = actual.swap_remove(index);
        }
    }
}

pub fn assert_no_error(results: &[CheckResult]) {
    let errors = error_results(results);
    assert!(
        errors.is_empty(),
        "expected no RS-HEXARCH-27 errors, got: {errors:#?}"
    );
}

pub fn assert_no_workspace_members_error_with_file_prefix(results: &[CheckResult], prefix: &str) {
    let errors = results
        .iter()
        .filter(|result| {
            result.id == WORKSPACE_MEMBERS_RULE_ID
                && result.severity == Severity::Error
                && result
                    .file
                    .as_deref()
                    .is_some_and(|file| file.starts_with(prefix))
        })
        .collect::<Vec<_>>();
    assert!(
        errors.is_empty(),
        "expected no {WORKSPACE_MEMBERS_RULE_ID} errors with file prefix {prefix}, got: {errors:#?}"
    );
}

pub fn assert_workspace_members_error_summary(
    results: &[CheckResult],
    expected_count: usize,
    expected_file: &str,
    title_contains: &[&str],
    message_contains: &[&str],
) {
    let errors = results
        .iter()
        .filter(|result| {
            result.id == WORKSPACE_MEMBERS_RULE_ID && result.severity == Severity::Error
        })
        .collect::<Vec<_>>();
    assert_eq!(
        errors.len(),
        expected_count,
        "unexpected {WORKSPACE_MEMBERS_RULE_ID} error count: {errors:#?}"
    );
    for result in &errors {
        assert_eq!(result.file.as_deref(), Some(expected_file), "{errors:#?}");
        assert!(
            title_contains
                .iter()
                .all(|needle| result.title.contains(needle)),
            "{errors:#?}"
        );
        assert!(
            message_contains
                .iter()
                .all(|needle| result.message.contains(needle)),
            "{errors:#?}"
        );
    }
}

pub fn assert_error_count(results: &[CheckResult], expected_count: usize) {
    let errors = error_results(results);
    assert_eq!(
        errors.len(),
        expected_count,
        "unexpected RS-HEXARCH-27 error count: {errors:#?}"
    );
}

pub fn assert_any_result_contains_title(results: &[CheckResult], needles: &[&str]) {
    let errors = error_results(results);
    for needle in needles {
        assert!(
            errors.iter().any(|result| result.title.contains(needle)),
            "missing expected title `{needle}` in RS-HEXARCH-27 results: {errors:#?}"
        );
    }
}

fn error_results(results: &[CheckResult]) -> Vec<&CheckResult> {
    results
        .iter()
        .filter(|result| result.id == RULE_ID && result.severity == Severity::Error)
        .collect()
}
