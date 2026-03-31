use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

const RULE_ID: &str = "RS-ARCH-16";

pub fn assert_error_files(results: &[CheckResult], expected: &[&str]) {
    let actual = results
        .iter()
        .filter(|result| result.id() == RULE_ID && result.severity() == Severity::Error)
        .filter_map(|result| result.file().map(str::to_owned))
        .collect::<BTreeSet<_>>();
    let expected = expected
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected, "unexpected {RULE_ID} error set: {results:#?}");
}
