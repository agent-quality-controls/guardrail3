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

const ID: &str = "RS-CODE-13";

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
    let actual = results
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

pub fn assert_inventories_todo_unimplemented_and_unreachable_macros_in_real_files(
    results: &[CheckResult],
    backend_rel: &str,
    worker_rel: &str,
    backend_line: usize,
    worker_line: usize,
) {
    let mut actual = findings(results);
    actual.sort_by(|left, right| {
        left.file
            .cmp(&right.file)
            .then(left.line.cmp(&right.line))
            .then(left.title.cmp(&right.title))
            .then(left.message.cmp(&right.message))
    });
    assert_value_eq(
        actual,
        vec![
            Finding {
                id: ID,
                severity: Severity::Warn,
                title: "todo! macro",
                message: "`todo!()` macro found: pub fn todo_probe() { todo!() }.",
                file: Some(backend_rel),
                line: Some(backend_line),
                inventory: false,
            },
            Finding {
                id: ID,
                severity: Severity::Warn,
                title: "unimplemented! macro",
                message: "`unimplemented!()` macro found: fn worker_probe() { unimplemented!(); unreachable!(); }.",
                file: Some(worker_rel),
                line: Some(worker_line),
                inventory: false,
            },
            Finding {
                id: ID,
                severity: Severity::Info,
                title: "unreachable! macro",
                message: "`unreachable!()` macro found: fn worker_probe() { unimplemented!(); unreachable!(); }.",
                file: Some(worker_rel),
                line: Some(worker_line),
                inventory: false,
            },
        ],
    );
}

pub fn assert_value_eq<A, B>(actual: A, expected: B)
where
    A: std::fmt::Debug + PartialEq<B>,
    B: std::fmt::Debug,
{
    assert_eq!(actual, expected);
}
