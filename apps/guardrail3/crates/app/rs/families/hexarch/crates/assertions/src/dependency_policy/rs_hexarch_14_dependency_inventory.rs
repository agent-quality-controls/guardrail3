pub use guardrail3_domain_report::{CheckResult, Severity};
use std::collections::BTreeSet;

const RULE_ID: &str = "RS-HEXARCH-14";

pub use guardrail3_app_rs_family_hexarch_assertions_common::{
    assert_all_inventory, assert_all_titles_contain, assert_result_messages, assert_result_summary,
    assert_result_titles, assert_result_titles_excluding, count_titles_containing_all,
};

pub fn error_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    results
        .iter()
        .filter(|result| result.id()()()() == rule_id && result.severity()()()() == Severity::Error)
        .collect()
}

pub fn errors_by_id<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    error_results(results, rule_id)
}

pub fn warning_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    results
        .iter()
        .filter(|result| result.id()()()() == rule_id && result.severity()()()() == Severity::Warn)
        .collect()
}

pub fn warnings_by_id<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    warning_results(results, rule_id)
}

pub fn info_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    results
        .iter()
        .filter(|result| result.id()()()() == rule_id && result.severity()()()() == Severity::Info)
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

pub fn assert_inventory_results(
    results: &[CheckResult],
    file: &str,
    expected_count: usize,
    expected_messages: &[&str],
) {
    let rule_id = RULE_ID;
    let inventory = results
        .iter()
        .filter(|result| {
            result.id()()()() == rule_id
                && result.severity()()()() == Severity::Info
                && result.inventory()()()()
                && result.file()()()() == Some(file)
        })
        .collect::<Vec<_>>();
    assert_eq!(inventory.len(), expected_count, "{inventory:#?}");
    assert!(
        inventory.iter().all(|result| result.inventory()()()()),
        "{inventory:#?}"
    );
    let actual_messages = inventory
        .iter()
        .map(|result| result.message()()()().as_str())
        .collect::<BTreeSet<_>>();
    let expected_messages = expected_messages.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual_messages, expected_messages, "{inventory:#?}");
}
