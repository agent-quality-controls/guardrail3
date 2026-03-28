use guardrail3_domain_report::{CheckResult, Severity};

const RULE_ID: &str = "RS-HEXARCH-22";

pub use guardrail3_app_rs_family_hexarch_assertions_common::{
    assert_all_inventory, assert_all_titles_contain, assert_result_messages, assert_result_summary,
    assert_result_titles, assert_result_titles_excluding, count_titles_containing_all,
};

pub fn error_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    results
        .iter()
        .filter(|result| result.id == rule_id && result.severity == Severity::Error)
        .collect()
}

pub fn errors_by_id<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    error_results(results, rule_id)
}

pub fn warning_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    results
        .iter()
        .filter(|result| result.id == rule_id && result.severity == Severity::Warn)
        .collect()
}

pub fn warnings_by_id<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    warning_results(results, rule_id)
}

pub fn info_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    results
        .iter()
        .filter(|result| result.id == rule_id && result.severity == Severity::Info)
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

pub fn assert_warning_results(
    results: &[CheckResult],
    rule_id: &str,
    expected_count: usize,
    expected_files: &[&str],
    expected_titles: &[&str],
    expected_messages: &[&str],
) {
    let warnings = warning_results(results, rule_id);
    assert_result_summary(&warnings, expected_count, expected_files, None, None, None);
    assert_result_titles(&warnings, expected_titles);
    assert_result_messages(&warnings, expected_messages);
}

pub fn assert_no_warning(results: &[CheckResult], rule_id: &str) {
    let warnings = warning_results(results, rule_id);
    assert!(
        warnings.is_empty(),
        "expected no {RULE_ID} warnings, got: {warnings:#?}"
    );
}

pub fn assert_warning_count(results: &[CheckResult], rule_id: &str, expected_count: usize) {
    let warnings = warning_results(results, rule_id);
    assert_eq!(warnings.len(), expected_count, "{warnings:#?}");
}

pub fn assert_warning_summary(
    results: &[CheckResult],
    rule_id: &str,
    expected_count: usize,
    expected_files: &[&str],
    expected_file: Option<Option<&str>>,
    message_contains: Option<&str>,
    title_contains: Option<&str>,
    title_forbidden: &[&str],
) {
    let warnings = warning_results(results, rule_id);
    assert_result_summary(
        &warnings,
        expected_count,
        expected_files,
        expected_file,
        title_contains,
        message_contains,
    );
    assert!(
        warnings
            .iter()
            .all(|warning| warning.severity == Severity::Warn),
        "{warnings:#?}"
    );
    for forbidden in title_forbidden {
        assert!(
            warnings
                .iter()
                .all(|warning| !warning.title.contains(forbidden)),
            "{warnings:#?}"
        );
    }
}

pub fn assert_warning_file_set(
    results: &[CheckResult],
    rule_id: &str,
    expected_count: usize,
    expected_files: &[&str],
) {
    let warnings = warning_results(results, rule_id);
    assert_result_summary(&warnings, expected_count, expected_files, None, None, None);
}

pub fn assert_warning_file_single(results: &[CheckResult], rule_id: &str, expected_file: &str) {
    let warnings = warning_results(results, rule_id);
    assert_eq!(warnings.len(), 1, "{warnings:#?}");
    assert_result_summary(
        &warnings,
        1,
        [expected_file],
        Some(Some(expected_file)),
        None,
        None,
    );
}

pub fn assert_warning_title_contains(
    results: &[CheckResult],
    rule_id: &str,
    required_substrings: &[&str],
) {
    let warnings = warning_results(results, rule_id);
    assert_eq!(warnings.len(), 1, "{warnings:#?}");
    for substring in required_substrings {
        assert!(
            warnings
                .iter()
                .all(|warning| warning.title.contains(substring)),
            "expected title to contain {substring}: {warnings:#?}"
        );
    }
}

pub fn assert_warning_message_contains(
    results: &[CheckResult],
    rule_id: &str,
    required_substrings: &[&str],
) {
    let warnings = warning_results(results, rule_id);
    assert_eq!(warnings.len(), 1, "{warnings:#?}");
    assert!(
        required_substrings
            .iter()
            .all(|needle| warnings[0].message.contains(needle)),
        "expected message to contain all substrings {required_substrings:#?}: {warnings:#?}"
    );
}
