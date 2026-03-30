pub use guardrail3_domain_report::{CheckResult, Severity};
use std::collections::BTreeSet;

const RULE_ID: &str = "RS-HEXARCH-19";

pub use guardrail3_app_rs_family_hexarch_assertions_common::{
    assert_all_inventory, assert_all_titles_contain, assert_result_messages, assert_result_summary,
    assert_result_titles, assert_result_titles_excluding, count_titles_containing_all,
};

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
    expected_titles: &[&str],
) {
    let errors = error_results(results, rule_id);
    assert_eq!(errors.len(), expected_count, "{errors:#?}");
    assert_result_titles(&errors, expected_titles);
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
    assert_result_summary(&errors, expected_count, expected_files, None, None, None);
}

pub fn assert_cycle_layers(
    cycle_layers: &[String],
    expected_count: usize,
    expected_layers: &[&str],
) {
    assert_eq!(cycle_layers.len(), expected_count, "{cycle_layers:#?}");
    let expected_layers = expected_layers
        .iter()
        .map(|layer| layer.to_string())
        .collect::<BTreeSet<_>>();
    let actual_layers = cycle_layers.iter().cloned().collect::<BTreeSet<_>>();
    assert_eq!(actual_layers, expected_layers, "{cycle_layers:#?}");
}

pub fn assert_error_result_summary(
    results: &[CheckResult],
    rule_id: &str,
    expected_count: usize,
    expected_files: &[&str],
    expected_file: Option<Option<&str>>,
    severity: Option<Severity>,
    title_contains: Option<&str>,
    message_contains: Option<&str>,
) {
    let errors = error_results(results, rule_id);
    assert_result_summary(
        &errors,
        expected_count,
        expected_files,
        expected_file,
        title_contains,
        message_contains,
    );
    if let Some(expected_severity) = severity {
        assert!(
            errors
                .iter()
                .all(|error| error.severity == expected_severity),
            "{errors:#?}"
        );
    }
}

pub fn assert_error_title_contains(
    results: &[CheckResult],
    rule_id: &str,
    required_substrings: &[&str],
) {
    let errors = error_results(results, rule_id);
    assert_eq!(errors.len(), 1, "{errors:#?}");
    for substring in required_substrings {
        assert!(
            errors
                .iter()
                .all(|result| result.title().contains(substring)),
            "expected title to contain {substring}: {errors:#?}"
        );
    }
}

pub fn assert_error_message_contains(
    results: &[CheckResult],
    rule_id: &str,
    required_substrings: &[&str],
) {
    let errors = error_results(results, rule_id);
    assert_eq!(errors.len(), 1, "{errors:#?}");
    for substring in required_substrings {
        assert!(
            errors
                .iter()
                .all(|result| result.message().contains(substring)),
            "expected message to contain {substring}: {errors:#?}"
        );
    }
}
