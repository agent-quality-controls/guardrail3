use guardrail3_domain_report::{CheckResult, Severity};

const RULE_ID: &str = "RS-HEXARCH-15";

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

pub fn assert_no_warning(results: &[CheckResult], rule_id: &str) {
    let rule_id = if rule_id.is_empty() {
        RULE_ID
    } else {
        rule_id
    };
    let warnings = warning_results(results, rule_id);
    assert!(
        warnings.is_empty(),
        "expected no {rule_id} warnings, got: {warnings:#?}"
    );
}

pub fn assert_titles(results: &[CheckResult], expected_titles: &[&str]) {
    let findings = results
        .iter()
        .filter(|result| result.id.is_empty())
        .collect::<Vec<_>>();
    assert_eq!(findings.len(), expected_titles.len(), "{findings:#?}");
    assert_result_titles(&findings, expected_titles);
}

pub fn assert_title_set(
    results: &[CheckResult],
    rule_id: &str,
    expected_count: usize,
    expected_titles: &[&str],
) {
    let warnings = warning_results(results, rule_id);
    assert_eq!(warnings.len(), expected_count, "{warnings:#?}");
    assert_result_titles(&warnings, expected_titles);
}

pub fn assert_result_title_contains(
    results: &[CheckResult],
    rule_id: &str,
    expected_count: usize,
    expected_files: &[&str],
    required_title: &str,
) {
    let warnings = warning_results(results, rule_id);
    assert_result_summary(
        &warnings,
        expected_count,
        expected_files,
        None,
        Some(required_title),
        None,
    );
}

pub fn assert_error_title_forbidden(results: &[CheckResult], rule_id: &str, forbidden: &[&str]) {
    let warnings = warning_results(results, rule_id);
    assert!(
        warnings
            .iter()
            .all(|result| forbidden.iter().all(|needle| !result.title.contains(needle))),
        "expected forbidden title text to be absent in {warnings:#?}"
    );
}
