pub use guardrail3_domain_report::{CheckResult, Severity};

const RULE_ID: &str = "RS-HEXARCH-03";

pub use guardrail3_app_rs_family_hexarch_assertions_common::{
    assert_all_inventory, assert_all_titles_contain, assert_result_messages, assert_result_summary,
    assert_result_titles, assert_result_titles_excluding, count_titles_containing_all,
};

fn resolve_rule_id(rule_id: &str) -> &str {
    if rule_id.is_empty() { RULE_ID } else { rule_id }
}

fn result_matches(
    result: &CheckResult,
    file: Option<&str>,
    file_prefix: Option<&str>,
    title_contains: &[&str],
    title_excludes: &[&str],
    message_contains: &[&str],
    message_excludes: &[&str],
) -> bool {
    if let Some(file) = file {
        if result.file.as_deref() != Some(file) {
            return false;
        }
    }

    if let Some(prefix) = file_prefix {
        if !result
            .file
            .as_deref()
            .is_none_or(|path| path.starts_with(prefix))
        {
            return false;
        }
    }

    title_contains
        .iter()
        .all(|needle| result.title.contains(needle))
        && title_excludes
            .iter()
            .all(|needle| !result.title.contains(needle))
        && message_contains
            .iter()
            .all(|needle| result.message.contains(needle))
        && message_excludes
            .iter()
            .all(|needle| !result.message.contains(needle))
}

fn matching_error_results<'a>(
    results: &'a [CheckResult],
    rule_id: &str,
    file: Option<&str>,
    file_prefix: Option<&str>,
    title_contains: &[&str],
    title_excludes: &[&str],
    message_contains: &[&str],
    message_excludes: &[&str],
) -> Vec<&'a CheckResult> {
    let rule_id = resolve_rule_id(rule_id);
    results
        .iter()
        .filter(|result| result.id == rule_id && result.severity == Severity::Error)
        .filter(|result| {
            result_matches(
                result,
                file,
                file_prefix,
                title_contains,
                title_excludes,
                message_contains,
                message_excludes,
            )
        })
        .collect()
}

pub fn error_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    let rule_id = resolve_rule_id(rule_id);
    results
        .iter()
        .filter(|result| result.id == rule_id && result.severity == Severity::Error)
        .collect()
}

pub fn errors_by_id<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    error_results(results, rule_id)
}

pub fn warning_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    let rule_id = resolve_rule_id(rule_id);
    results
        .iter()
        .filter(|result| result.id == rule_id && result.severity == Severity::Warn)
        .collect()
}

pub fn warnings_by_id<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    warning_results(results, rule_id)
}

pub fn info_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    let rule_id = resolve_rule_id(rule_id);
    results
        .iter()
        .filter(|result| result.id == rule_id && result.severity == Severity::Info)
        .collect()
}

pub fn infos_by_id<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    info_results(results, rule_id)
}

pub fn assert_no_error(results: &[CheckResult], rule_id: &str) {
    let rule_id = resolve_rule_id(rule_id);
    let errors = error_results(results, rule_id);
    assert!(
        errors.is_empty(),
        "expected no {rule_id} errors, got: {errors:#?}"
    );
}

pub fn assert_error_count(results: &[CheckResult], rule_id: &str, expected_count: usize) {
    let rule_id = resolve_rule_id(rule_id);
    let errors = error_results(results, rule_id);
    assert_eq!(
        errors.len(),
        expected_count,
        "unexpected {rule_id} error count: {errors:#?}"
    );
}

pub fn assert_error_count_matching(
    results: &[CheckResult],
    rule_id: &str,
    expected_count: usize,
    file: Option<&str>,
    file_prefix: Option<&str>,
    title_contains: &[&str],
    title_excludes: &[&str],
    message_contains: &[&str],
    message_excludes: &[&str],
) {
    let rule_id = resolve_rule_id(rule_id);
    let errors = matching_error_results(
        results,
        rule_id,
        file,
        file_prefix,
        title_contains,
        title_excludes,
        message_contains,
        message_excludes,
    );
    assert_eq!(
        errors.len(),
        expected_count,
        "unexpected {rule_id} matches: {errors:#?}"
    );
}

pub fn assert_error_count_matching_file(
    results: &[CheckResult],
    rule_id: &str,
    file: &str,
    expected_count: usize,
    title_contains: &[&str],
    title_excludes: &[&str],
    message_contains: &[&str],
    message_excludes: &[&str],
) {
    assert_error_count_matching(
        results,
        rule_id,
        expected_count,
        Some(file),
        None,
        title_contains,
        title_excludes,
        message_contains,
        message_excludes,
    );
}

pub fn assert_error_count_matching_prefix(
    results: &[CheckResult],
    rule_id: &str,
    file_prefix: &str,
    expected_count: usize,
    title_contains: &[&str],
    title_excludes: &[&str],
    message_contains: &[&str],
    message_excludes: &[&str],
) {
    assert_error_count_matching(
        results,
        rule_id,
        expected_count,
        None,
        Some(file_prefix),
        title_contains,
        title_excludes,
        message_contains,
        message_excludes,
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
    let rule_id = resolve_rule_id(rule_id);
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
                    .all(|needle| result.title.contains(needle))
            }),
            "unexpected {rule_id} titles: {errors:#?}"
        );
    }

    if let Some(title_excludes) = title_excludes {
        assert!(
            errors.iter().all(|result| {
                title_excludes
                    .iter()
                    .all(|needle| !result.title.contains(needle))
            }),
            "unexpected {rule_id} titles: {errors:#?}"
        );
    }

    if let Some(message_contains) = message_contains {
        assert!(
            errors.iter().all(|result| {
                message_contains
                    .iter()
                    .all(|needle| result.message.contains(needle))
            }),
            "unexpected {rule_id} messages: {errors:#?}"
        );
    }
}

pub fn assert_no_error_at_path(results: &[CheckResult], rule_id: &str, file: &str) {
    let rule_id = resolve_rule_id(rule_id);
    let errors = error_results(results, rule_id);
    assert!(
        errors
            .iter()
            .all(|result| result.file.as_deref() != Some(file)),
        "expected no {rule_id} errors at {file}, got: {errors:#?}"
    );
}

pub fn assert_no_error_with_file_prefix(results: &[CheckResult], rule_id: &str, prefix: &str) {
    let rule_id = resolve_rule_id(rule_id);
    let errors = error_results(results, rule_id);
    assert!(
        errors.iter().all(|result| result
            .file
            .as_deref()
            .is_none_or(|file| !file.starts_with(prefix))),
        "expected no {rule_id} errors with file prefix {prefix}, got: {errors:#?}"
    );
}
