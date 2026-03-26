use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

const RULE_ID: &str = "RS-ARCH-08";

pub fn info_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    results
        .iter()
        .filter(|result| {
            result.id == resolved_rule_id(rule_id) && result.severity == Severity::Info
        })
        .collect()
}

pub fn assert_info_files(results: &[CheckResult], rule_id: &str, expected: &[&str]) {
    let actual = info_results(results, rule_id)
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

fn resolved_rule_id(rule_id: &str) -> &str {
    if rule_id.is_empty() { RULE_ID } else { rule_id }
}
