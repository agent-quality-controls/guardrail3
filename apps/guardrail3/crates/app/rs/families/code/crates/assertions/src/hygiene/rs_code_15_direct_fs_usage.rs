use std::collections::BTreeSet;

pub use guardrail3_domain_report::{CheckResult, Severity};

pub use crate::finding_support::{Finding, RuleFinding};

const ID: &str = "RS-CODE-15";

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

pub fn assert_attacks_direct_std_fs_imports_and_calls_in_real_owned_files_with_exact_metadata(
    results: &[CheckResult],
    backend_rel: &str,
    worker_rel: &str,
    backend_import_line: usize,
    backend_call_line: usize,
    worker_import_line: usize,
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
                severity: Severity::Error,
                title: "direct std::fs import",
                message: "Direct `use std::fs` import found: `use std::fs;`.",
                file: Some(backend_rel),
                line: Some(backend_import_line),
                inventory: false,
            },
            Finding {
                id: ID,
                severity: Severity::Error,
                title: "direct std::fs call",
                message: "Direct `std::fs::*` call found: `let _ = std::fs::read_to_string(\"backend.txt\");`.",
                file: Some(backend_rel),
                line: Some(backend_call_line),
                inventory: false,
            },
            Finding {
                id: ID,
                severity: Severity::Error,
                title: "direct std::fs import",
                message: "Direct `use std::fs` import found: `use std::{fs, io}; fn fs_call_probe() { let _ = std::fs::read_to_string(\"jobs.txt\"); }`.",
                file: Some(worker_rel),
                line: Some(worker_import_line),
                inventory: false,
            },
        ],
    );
}

pub fn assert_prefers_import_hit_when_import_and_call_share_one_line(
    results: &[CheckResult],
    rel: &str,
    line: usize,
) {
    let actual = findings(results);
    assert_value_eq(
        actual,
        vec![Finding {
            id: ID,
            severity: Severity::Error,
            title: "direct std::fs import",
            message: "Direct `use std::fs` import found: `use std::fs; fn same_line_probe() { let _ = std::fs::read_to_string(\"same-line.txt\"); }`.",
            file: Some(rel),
            line: Some(line),
            inventory: false,
        }],
    );
}

pub fn assert_value_eq<A, B>(actual: A, expected: B)
where
    A: std::fmt::Debug + PartialEq<B>,
    B: std::fmt::Debug,
{
    assert_eq!(actual, expected);
}
