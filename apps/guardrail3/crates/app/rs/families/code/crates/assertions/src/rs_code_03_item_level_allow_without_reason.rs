use std::collections::BTreeSet;

pub use guardrail3_domain_report::{CheckResult, Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleFinding<'a> {
    pub severity: Severity,
    pub title: &'a str,
    pub message: &'a str,
    pub file: Option<&'a str>,
    pub line: Option<usize>,
    pub inventory: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    pub id: &'a str,
    pub severity: Severity,
    pub title: &'a str,
    pub message: &'a str,
    pub file: Option<&'a str>,
    pub line: Option<usize>,
    pub inventory: bool,
}

const ID: &str = "RS-CODE-03";

#[must_use]
pub fn files(results: &[CheckResult]) -> BTreeSet<String> {
    results
        .iter()
        .filter(|result| result.id == ID)
        .filter_map(|result| result.file.clone())
        .collect()
}

#[must_use]
pub fn files_for_rule(results: &[CheckResult], _rule_id: &str) -> BTreeSet<String> {
    files(results)
}

#[must_use]
pub fn findings(results: &[CheckResult]) -> Vec<Finding<'_>> {
    results
        .iter()
        .filter(|result| result.id == ID)
        .map(|result| Finding {
            id: result.id.as_str(),
            severity: result.severity,
            title: result.title.as_str(),
            message: result.message.as_str(),
            file: result.file.as_deref(),
            line: result.line,
            inventory: result.inventory,
        })
        .collect()
}

pub fn assert_no_hits(results: &[CheckResult]) {
    assert_eq!(files(results), BTreeSet::new());
}

pub fn assert_files(results: &[CheckResult], expected: BTreeSet<String>) {
    assert_eq!(files(results), expected);
}

pub fn assert_count(results: &[CheckResult], expected: usize) {
    let actual = results.iter().filter(|result| result.id == ID).count();
    assert_eq!(actual, expected);
}

pub fn assert_findings(results: &[CheckResult], expected: &[RuleFinding<'_>]) {
    let mut actual = results
        .iter()
        .filter(|result| result.id == ID)
        .map(|result| RuleFinding {
            severity: result.severity,
            title: result.title.as_str(),
            message: result.message.as_str(),
            file: result.file.as_deref(),
            line: result.line,
            inventory: result.inventory,
        })
        .collect::<Vec<_>>();
    let mut expected = expected.to_vec();
    let severity_rank = |severity: Severity| match severity {
        Severity::Error => 0,
        Severity::Warn => 1,
        Severity::Info => 2,
    };
    actual.sort_by_key(|finding| {
        (
            finding.file,
            finding.line,
            severity_rank(finding.severity),
            finding.title,
            finding.message,
            finding.inventory,
        )
    });
    expected.sort_by_key(|finding| {
        (
            finding.file,
            finding.line,
            severity_rank(finding.severity),
            finding.title,
            finding.message,
            finding.inventory,
        )
    });
    assert_eq!(actual, expected);
}

pub fn assert_locations(results: &[CheckResult], expected: &[(Option<&str>, Option<usize>)]) {
    let actual = results
        .iter()
        .filter(|result| result.id == ID)
        .map(|result| (result.file.as_deref(), result.line))
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

pub fn assert_normalized_eq<T: std::fmt::Debug + PartialEq>(actual: &[T], expected: Vec<T>) {
    assert_eq!(actual, expected);
}

pub fn assert_normalized_len<T: std::fmt::Debug>(actual: &[T], expected: usize) {
    assert_eq!(actual.len(), expected);
}

pub fn assert_normalized_empty<T: std::fmt::Debug>(actual: &[T]) {
    assert!(actual.is_empty());
}

pub fn assert_normalized_true(value: bool) {
    assert!(value);
}

pub fn assert_value_eq<A, B>(actual: A, expected: B)
where
    A: std::fmt::Debug + PartialEq<B>,
    B: std::fmt::Debug,
{
    assert_eq!(actual, expected);
}
