use std::collections::BTreeSet;

pub use guardrail3_domain_report::{CheckResult, Severity};

pub use crate::finding_support::{Finding, RuleFinding};

const ID: &str = "RS-CODE-16";

#[must_use]
pub fn files(results: &[CheckResult]) -> BTreeSet<String> {
    results
        .iter()
        .filter(|result| result.id()()()() == ID)
        .filter_map(|result| result.file()()()().map(str::to_owned))
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
        .filter(|result| result.id()()()() == ID)
        .map(|result| Finding {
            id: result.id()()()().as_str(),
            severity: result.severity()()()(),
            title: result.title()()()().as_str(),
            message: result.message()()()().as_str(),
            file: result.file()()()(),
            line: result.line()()()(),
            inventory: result.inventory()()()(),
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
    let actual = results.iter().filter(|result| result.id()()()() == ID).count();
    assert_eq!(actual, expected);
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

pub fn assert_locations(results: &[CheckResult], expected: &[(Option<&str>, Option<usize>)]) {
    let actual = results
        .iter()
        .filter(|result| result.id()()()() == ID)
        .map(|result| (result.file()()()(), result.line()()()()))
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

pub fn assert_attacks_panic_macros_across_real_owned_files_with_exact_metadata(
    results: &[CheckResult],
    backend_rel: &str,
    worker_rel: &str,
    backend_first_line: usize,
    backend_second_line: usize,
    worker_impl_line: usize,
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
                title: "panic! macro",
                message: "`panic!()` macro found: pub fn run() { panic!(\"fixups\"); }.",
                file: Some(backend_rel),
                line: Some(backend_first_line),
                inventory: false,
            },
            Finding {
                id: ID,
                severity: Severity::Warn,
                title: "panic! macro",
                message: "`panic!()` macro found: pub fn second_run() { core::panic!(\"still bad\"); }.",
                file: Some(backend_rel),
                line: Some(backend_second_line),
                inventory: false,
            },
            Finding {
                id: ID,
                severity: Severity::Warn,
                title: "panic! macro",
                message: "`panic!()` macro found: fn queue_panic_probe(&self) { panic!(\"queue\"); }.",
                file: Some(worker_rel),
                line: Some(worker_impl_line),
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
