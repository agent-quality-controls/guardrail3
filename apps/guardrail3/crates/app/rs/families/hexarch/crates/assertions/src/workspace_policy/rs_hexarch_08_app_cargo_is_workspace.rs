pub use guardrail3_domain_report::{CheckResult, Severity};

const RULE_ID: &str = "RS-HEXARCH-08";

pub use guardrail3_app_rs_family_hexarch_assertions_common::{
    assert_all_inventory, assert_all_titles_contain, assert_result_messages, assert_result_summary,
    assert_result_titles, assert_result_titles_excluding, count_titles_containing_all,
};

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
    let mut actual = error_results(results, RULE_ID);
    assert_eq!(
        actual.len(),
        expected.len(),
        "unexpected RS-HEXARCH-08 results: {actual:#?}"
    );

    for expected_result in expected {
        let index = actual.iter().position(|result| {
            expected_result
                .file
                .is_none_or(|file| result.file() == Some(file))
                && expected_result.file_contains.is_none_or(|expected_file| {
                    result
                        .file
                        .as_deref()
                        .is_some_and(|actual_file| actual_file.contains(expected_file))
                })
                && expected_result.title_contains.is_none_or(|needles| {
                    needles.iter().all(|needle| result.title().contains(needle))
                })
                && expected_result.message_contains.is_none_or(|needles| {
                    needles.iter().all(|needle| result.message().contains(needle))
                })
        });

        assert!(
            index.is_some(),
            "missing expected RS-HEXARCH-08 result {expected_result:#?}; actual: {actual:#?}"
        );
        if let Some(index) = index {
            let _ = actual.swap_remove(index);
        }
    }
}

pub fn assert_any_result_contains_title(results: &[CheckResult], needles: &[&str]) {
    let errors = error_results(results, RULE_ID);
    for needle in needles {
        assert!(
            errors.iter().any(|result| result.title().contains(needle)),
            "missing expected title `{needle}` in RS-HEXARCH-08 results: {errors:#?}"
        );
    }
}

pub fn assert_no_error_file_contains(results: &[CheckResult], file: &str) {
    let errors = error_results(results, RULE_ID);
    assert!(
        errors
            .iter()
            .all(|result| !result.file().unwrap_or("").contains(file)),
        "found RS-HEXARCH-08 result in path `{file}`: {errors:#?}"
    );
}

pub fn error_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    results
        .iter()
        .filter(|result| result.id() == rule_id && result.severity() == Severity::Error)
        .collect()
}

pub fn errors_by_id<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    error_results(results, rule_id)
}

pub fn warning_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    results
        .iter()
        .filter(|result| result.id() == rule_id && result.severity() == Severity::Warn)
        .collect()
}

pub fn warnings_by_id<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    warning_results(results, rule_id)
}

pub fn info_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    results
        .iter()
        .filter(|result| result.id() == rule_id && result.severity() == Severity::Info)
        .collect()
}

pub fn infos_by_id<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    info_results(results, rule_id)
}

pub fn assert_no_error(results: &[CheckResult], rule_id: &str) {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    let errors = error_results(results, rule_id);
    assert!(
        errors.is_empty(),
        "expected no {rule_id} errors, got: {errors:#?}"
    );
}

pub fn assert_error_count(results: &[CheckResult], rule_id: &str, expected_count: usize) {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    let errors = error_results(results, rule_id);
    assert_eq!(
        errors.len(),
        expected_count,
        "unexpected {rule_id} error count: {errors:#?}"
    );
}

pub fn assert_error_summary<I>(
    results: &[CheckResult],
    rule_id: &str,
    expected_count: usize,
    expected_files: I,
    expected_file: Option<Option<&str>>,
    title_contains: Option<&[&str]>,
    title_excludes: Option<&[&str]>,
    message_contains: Option<&[&str]>,
) where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    let errors = error_results(results, rule_id);
    assert_result_summary(
        &errors,
        expected_count,
        expected_files,
        expected_file,
        None,
        None,
    );

    if let Some(title_contains) = title_contains {
        assert!(
            errors.iter().all(|result| {
                title_contains
                    .iter()
                    .all(|needle| result.title().contains(needle))
            }),
            "unexpected {rule_id} titles: {errors:#?}"
        );
    }

    if let Some(title_excludes) = title_excludes {
        assert!(
            errors.iter().all(|result| {
                title_excludes
                    .iter()
                    .all(|needle| !result.title().contains(needle))
            }),
            "unexpected {rule_id} titles: {errors:#?}"
        );
    }

    if let Some(message_contains) = message_contains {
        assert!(
            errors.iter().all(|result| {
                message_contains
                    .iter()
                    .all(|needle| result.message().contains(needle))
            }),
            "unexpected {rule_id} messages: {errors:#?}"
        );
    }
}

pub fn assert_no_error_at_path(results: &[CheckResult], rule_id: &str, file: &str) {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    let errors = error_results(results, rule_id);
    assert!(
        errors
            .iter()
            .all(|result| result.file() != Some(file)),
        "expected no {rule_id} errors at {file}, got: {errors:#?}"
    );
}

pub fn assert_no_error_with_file_prefix(results: &[CheckResult], rule_id: &str, prefix: &str) {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    let errors = error_results(results, rule_id);
    assert!(
        errors.iter().all(|result| result
            .file
            .as_deref()
            .is_none_or(|file| !file.starts_with(prefix))),
        "expected no {rule_id} errors with file prefix {prefix}, got: {errors:#?}"
    );
}
