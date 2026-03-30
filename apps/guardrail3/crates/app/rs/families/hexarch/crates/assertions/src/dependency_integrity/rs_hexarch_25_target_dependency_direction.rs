pub use guardrail3_domain_report::{CheckResult, Severity};
use std::collections::BTreeSet;

const RULE_ID: &str = "RS-HEXARCH-25";

pub use guardrail3_app_rs_family_hexarch_assertions_common::{
    assert_all_inventory, assert_all_titles_contain, assert_result_messages, assert_result_summary,
    assert_result_titles, assert_result_titles_excluding, count_titles_containing_all,
};
pub type ProjectTree = guardrail3_domain_project_tree::ProjectTree;

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

pub fn assert_error_results(
    results: &[CheckResult],
    rule_id: &str,
    expected_count: usize,
    expected_files: &[&str],
    expected_titles: &[&str],
) {
    let errors = error_results(results, rule_id);
    assert_eq!(errors.len(), expected_count, "{errors:#?}");
    let actual_files = errors
        .iter()
        .filter_map(|result| result.file())
        .collect::<BTreeSet<_>>();
    let expected_files = expected_files.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual_files, expected_files, "{errors:#?}");
    let actual_titles = errors
        .iter()
        .map(|result| result.title())
        .collect::<BTreeSet<_>>();
    let expected_titles = expected_titles.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual_titles, expected_titles, "{errors:#?}");
}

pub fn assert_error_count(results: &[CheckResult], rule_id: &str, expected_count: usize) {
    let errors = error_results(results, rule_id);
    assert_eq!(errors.len(), expected_count, "{errors:#?}");
}

pub fn assert_error_file_set(
    results: &[CheckResult],
    rule_id: &str,
    expected_count: usize,
    expected_files: &[&str],
) {
    let errors = error_results(results, rule_id);
    assert_eq!(errors.len(), expected_count, "{errors:#?}");
    let actual_files = errors
        .iter()
        .filter_map(|result| result.file())
        .collect::<BTreeSet<_>>();
    let expected_files = expected_files.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual_files, expected_files, "{errors:#?}");
}
