pub use guardrail3_domain_report::{CheckResult, Severity};

pub use crate::finding_support::{Finding, RuleFinding};

const ID: &str = "RS-CODE-32";

#[must_use]
pub fn findings(results: &[CheckResult]) -> Vec<Finding<'_>> {
    results
        .iter()
        .filter(|result| result.id() == ID)
        .map(|result| Finding {
            id: result.id(),
            severity: result.severity(),
            title: result.title(),
            message: result.message(),
            file: result.file(),
            line: result.line(),
            inventory: result.inventory(),
        })
        .collect()
}

pub fn assert_no_hits(results: &[CheckResult]) {
    assert!(findings(results).is_empty());
}

pub fn assert_findings(results: &[CheckResult], expected: &[RuleFinding<'_>]) {
    let actual = results
        .iter()
        .filter(|result| result.id() == ID)
        .map(|result| RuleFinding {
            severity: result.severity(),
            title: result.title(),
            message: result.message(),
            file: result.file(),
            line: result.line(),
            inventory: result.inventory(),
        })
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

pub fn assert_normalized_len<T: std::fmt::Debug>(actual: &[T], expected: usize) {
    assert_eq!(actual.len(), expected);
}
