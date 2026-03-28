use guardrail3_domain_report::{CheckResult, Severity};

const RULE_ID: &str = "RS-HEXARCH-26";

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

pub fn assert_error_summary(
    results: &[CheckResult],
    rule_id: &str,
    expected_count: usize,
    expected_files: &[&str],
    message_contains: Option<&str>,
) {
    let errors = error_results(results, rule_id);
    assert_eq!(
        errors.len(),
        expected_count,
        "unexpected {RULE_ID} error count: {errors:#?}"
    );

    let actual_files = errors
        .iter()
        .filter_map(|result| result.file.as_deref())
        .collect::<std::collections::BTreeSet<_>>();
    let expected_files = expected_files
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(actual_files, expected_files, "{errors:#?}");

    if let Some(message_contains) = message_contains {
        assert!(
            errors
                .iter()
                .all(|result| result.message.contains(message_contains)),
            "{errors:#?}"
        );
    }
}

pub fn assert_no_error(results: &[CheckResult], rule_id: &str) {
    let errors = error_results(results, rule_id);
    assert!(
        errors.is_empty(),
        "expected no {RULE_ID} errors, got: {errors:#?}"
    );
}
