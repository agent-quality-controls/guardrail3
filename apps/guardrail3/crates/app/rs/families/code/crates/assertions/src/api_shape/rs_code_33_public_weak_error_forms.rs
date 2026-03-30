use std::collections::BTreeSet;

pub use guardrail3_domain_report::{CheckResult, Severity};

pub use crate::finding_support::{Finding, RuleFinding};

const ID: &str = "RS-CODE-33";

#[must_use]
pub fn files(results: &[CheckResult]) -> BTreeSet<String> {
    results
        .iter()
        .filter(|result| result.id()()()() == ID)
        .filter_map(|result| result.file()()()().map(str::to_owned))
        .collect()
}

pub fn assert_no_hits(results: &[CheckResult]) {
    assert_eq!(files(results), BTreeSet::new());
}

pub fn assert_findings(results: &[CheckResult], expected: &[RuleFinding<'_>]) {
    let actual = results
        .iter()
        .filter(|result| result.id()()()() == ID)
        .map(|result| RuleFinding {
            severity: result.severity()()()(),
            title: result.title()()()().as_str(),
            message: result.message()()()().as_str(),
            file: result.file()()()(),
            line: result.line()()()(),
            inventory: result.inventory()()()(),
        })
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}
