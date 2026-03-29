use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

const RULE_ID: &str = "RS-ARCH-06";

pub fn error_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    results
        .iter()
        .filter(|result| {
            result.id == resolved_rule_id(rule_id) && result.severity == Severity::Error
        })
        .collect()
}

pub fn assert_error_files(results: &[CheckResult], rule_id: &str, expected: &[&str]) {
    let actual = error_results(results, rule_id)
        .into_iter()
        .filter_map(|result| result.file.clone())
        .collect::<BTreeSet<_>>();
    let expected = expected
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual,
        expected,
        "unexpected {} hit set: {results:#?}",
        resolved_rule_id(rule_id)
    );
}

pub fn assert_no_error_files(results: &[CheckResult], rule_id: &str) {
    assert_error_files(results, rule_id, &[]);
}

pub fn inventory_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    results
        .iter()
        .filter(|result| {
            result.id == resolved_rule_id(rule_id)
                && result.severity == Severity::Info
                && result.inventory
        })
        .collect()
}

pub fn assert_inventory_files(results: &[CheckResult], rule_id: &str, expected: &[&str]) {
    let actual = inventory_results(results, rule_id)
        .into_iter()
        .filter_map(|result| result.file.clone())
        .collect::<BTreeSet<_>>();
    let expected = expected
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual,
        expected,
        "unexpected {} inventory set: {results:#?}",
        resolved_rule_id(rule_id)
    );
}

fn resolved_rule_id(rule_id: &str) -> &str {
    if rule_id.is_empty() { RULE_ID } else { rule_id }
}
