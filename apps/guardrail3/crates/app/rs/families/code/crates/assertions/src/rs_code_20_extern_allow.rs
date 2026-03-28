use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

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

const ID: &str = "RS-CODE-20";

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

pub fn assert_errors_on_allow_attr_on_extern_block(
    results: &[CheckResult],
    file: &str,
    line: usize,
) {
    let actual = findings(results);
    assert_value_eq(
        actual,
        vec![Finding {
            id: ID,
            severity: Severity::Error,
            title: "allow on extern block",
            message: "`#[allow(improper_ctypes)]` on an `extern` block hides FFI risk behind a broad suppression.",
            file: Some(file),
            line: Some(line),
            inventory: false,
        }],
    );
}

pub fn assert_errors_on_multiple_lints_from_one_extern_block_allow_attribute(
    results: &[CheckResult],
    file: &str,
    line: usize,
) {
    let mut actual = findings(results);
    actual.sort_by(|left, right| left.message.cmp(right.message));
    let mut expected = vec![
        Finding {
            id: ID,
            severity: Severity::Error,
            title: "allow on extern block",
            message: "`#[allow(clippy::all)]` on an `extern` block hides FFI risk behind a broad suppression.",
            file: Some(file),
            line: Some(line),
            inventory: false,
        },
        Finding {
            id: ID,
            severity: Severity::Error,
            title: "allow on extern block",
            message: "`#[allow(improper_ctypes)]` on an `extern` block hides FFI risk behind a broad suppression.",
            file: Some(file),
            line: Some(line),
            inventory: false,
        },
    ];
    expected.sort_by(|left, right| left.message.cmp(right.message));
    assert_value_eq(actual, expected);
}

pub fn assert_errors_when_stacked_allow_attrs_cover_the_same_extern_block(
    results: &[CheckResult],
    file: &str,
    line_one: usize,
    line_two: usize,
) {
    let actual = findings(results);
    assert_value_eq(
        actual,
        vec![
            Finding {
                id: ID,
                severity: Severity::Error,
                title: "allow on extern block",
                message: "`#[allow(improper_ctypes)]` on an `extern` block hides FFI risk behind a broad suppression.",
                file: Some(file),
                line: Some(line_one),
                inventory: false,
            },
            Finding {
                id: ID,
                severity: Severity::Error,
                title: "allow on extern block",
                message: "`#[allow(improper_ctypes_definitions)]` on an `extern` block hides FFI risk behind a broad suppression.",
                file: Some(file),
                line: Some(line_two),
                inventory: false,
            },
        ],
    );
}

pub fn assert_errors_on_cfg_attr_allow_covers_an_extern_block(
    results: &[CheckResult],
    file: &str,
    line: usize,
) {
    let actual = findings(results);
    assert_value_eq(
        actual,
        vec![Finding {
            id: ID,
            severity: Severity::Error,
            title: "allow on extern block",
            message: "`#[cfg_attr(..., allow(improper_ctypes))]` on an `extern` block hides FFI risk behind a broad suppression.",
            file: Some(file),
            line: Some(line),
            inventory: false,
        }],
    );
}

pub fn assert_errors_on_multiple_lints_from_one_cfg_attr_allow_on_extern_block(
    results: &[CheckResult],
    file: &str,
    line: usize,
) {
    let mut actual = findings(results);
    actual.sort_by(|left, right| left.message.cmp(right.message));
    let mut expected = vec![
        Finding {
            id: ID,
            severity: Severity::Error,
            title: "allow on extern block",
            message: "`#[cfg_attr(..., allow(clippy::all))]` on an `extern` block hides FFI risk behind a broad suppression.",
            file: Some(file),
            line: Some(line),
            inventory: false,
        },
        Finding {
            id: ID,
            severity: Severity::Error,
            title: "allow on extern block",
            message: "`#[cfg_attr(..., allow(improper_ctypes))]` on an `extern` block hides FFI risk behind a broad suppression.",
            file: Some(file),
            line: Some(line),
            inventory: false,
        },
    ];
    expected.sort_by(|left, right| left.message.cmp(right.message));
    assert_value_eq(actual, expected);
}

pub fn assert_errors_on_mixed_allow_and_cfg_attr_on_the_same_extern_block(
    results: &[CheckResult],
    file: &str,
    allow_line: usize,
    cfg_attr_line: usize,
) {
    let actual = findings(results);
    assert_value_eq(
        actual,
        vec![
            Finding {
                id: ID,
                severity: Severity::Error,
                title: "allow on extern block",
                message: "`#[allow(improper_ctypes)]` on an `extern` block hides FFI risk behind a broad suppression.",
                file: Some(file),
                line: Some(allow_line),
                inventory: false,
            },
            Finding {
                id: ID,
                severity: Severity::Error,
                title: "allow on extern block",
                message: "`#[cfg_attr(..., allow(improper_ctypes_definitions))]` on an `extern` block hides FFI risk behind a broad suppression.",
                file: Some(file),
                line: Some(cfg_attr_line),
                inventory: false,
            },
        ],
    );
}

pub fn assert_inventory_allow_attrs_on_extern_blocks(
    results: &[CheckResult],
    files: [&str; 3],
    lines: [usize; 3],
) {
    let actual = findings(results);
    assert_value_eq(
        actual,
        vec![
            Finding {
                id: ID,
                severity: Severity::Error,
                title: "allow on extern block",
                message: "`#[allow(improper_ctypes)]` on an `extern` block hides FFI risk behind a broad suppression.",
                file: Some(files[0]),
                line: Some(lines[0]),
                inventory: false,
            },
            Finding {
                id: ID,
                severity: Severity::Error,
                title: "allow on extern block",
                message: "`#[allow(improper_ctypes_definitions)]` on an `extern` block hides FFI risk behind a broad suppression.",
                file: Some(files[1]),
                line: Some(lines[1]),
                inventory: false,
            },
            Finding {
                id: ID,
                severity: Severity::Error,
                title: "allow on extern block",
                message: "`#[cfg_attr(..., allow(improper_ctypes))]` on an `extern` block hides FFI risk behind a broad suppression.",
                file: Some(files[2]),
                line: Some(lines[2]),
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
