use std::collections::{BTreeMap, BTreeSet};

use guardrail3_domain_report::CheckResult;

const ID: &str = "RS-CLIPPY-01";

pub use guardrail3_domain_report::Severity;

pub fn assert_multi_root_coverage(
    results: &[CheckResult],
    expected: &[(&str, Severity, bool, Option<&str>, &str)],
) {
    let coverage = results
        .iter()
        .filter(|result| result.id() == ID)
        .collect::<Vec<_>>();

    let actual = coverage
        .iter()
        .map(|result| {
            (
                result.message().to_owned(),
                (
                    result.severity(),
                    result.inventory(),
                    result.file().map(str::to_owned),
                    result.title().to_owned(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let expected = expected
        .iter()
        .map(|(message, severity, inventory, file, title)| {
            (
                (*message).to_owned(),
                (
                    *severity,
                    *inventory,
                    file.map(str::to_owned),
                    (*title).to_owned(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();

    assert_eq!(actual, expected);
}

pub fn assert_excludes_non_rust_roots(results: &[CheckResult]) {
    let coverage_messages = results
        .iter()
        .filter(|result| result.id() == ID)
        .map(|result| result.message())
        .collect::<Vec<_>>();

    assert!(
        coverage_messages.iter().all(
            |message| !message.contains("apps/landing") && !message.contains("packages/ui-kit")
        ),
        "expected non-Rust roots to stay out of clippy coverage results: {coverage_messages:#?}"
    );
}

pub fn assert_selective_uncovered(
    results: &[CheckResult],
    expected_messages: &[&str],
    expected_error_files: &[&str],
) {
    let coverage = results
        .iter()
        .filter(|result| result.id() == ID)
        .collect::<Vec<_>>();
    let actual_messages = coverage
        .iter()
        .map(|result| result.message().to_owned())
        .collect::<BTreeSet<_>>();
    let expected_messages = expected_messages
        .iter()
        .map(|message| (*message).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_messages, expected_messages);

    let errors = coverage
        .iter()
        .filter(|result| result.severity() == Severity::Error)
        .collect::<Vec<_>>();
    assert_eq!(
        errors.len(),
        expected_error_files.len(),
        "expected exactly the uncovered roots to error: {errors:#?}"
    );
    assert!(errors.iter().all(|result| !result.inventory()));
    assert_eq!(
        errors
            .iter()
            .filter_map(|result| result.file())
            .collect::<BTreeSet<_>>(),
        expected_error_files
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
    );
}

pub fn assert_root_failure(results: &[CheckResult], cargo_rel_path: &str, root_label: &str) {
    let coverage = results
        .iter()
        .filter(|result| result.id() == ID)
        .collect::<Vec<_>>();
    assert_eq!(coverage.len(), 1);
    let result = coverage[0];
    assert_eq!(result.severity(), Severity::Error);
    assert_eq!(result.title(), "Rust unit coverage could not be determined");
    assert_eq!(result.file(), Some(cargo_rel_path));
    assert!(
        result.message().contains(root_label),
        "expected root label in message: {result:#?}"
    );
    assert!(
        result.message().contains(cargo_rel_path),
        "expected cargo path in message: {result:#?}"
    );
    assert!(!result.inventory());
}

pub fn assert_no_coverage_results(results: &[CheckResult]) {
    let coverage = results
        .iter()
        .filter(|result| result.id() == ID)
        .collect::<Vec<_>>();

    assert!(
        coverage.is_empty(),
        "expected no clippy coverage results: {coverage:#?}"
    );
}

pub fn assert_unparseable_routed_cargo_root(results: &[CheckResult], cargo_rel_path: &str) {
    let failures = results
        .iter()
        .filter(|result| {
            result.id() == ID
                && result.severity() == Severity::Error
                && result.title() == "Rust unit coverage could not be determined"
        })
        .collect::<Vec<_>>();

    assert_eq!(failures.len(), 1);
    let result = failures[0];
    assert_eq!(result.file(), Some(cargo_rel_path));
    assert!(
        result
            .message()
            .contains("routed Cargo root `apps/backend` could not be parsed"),
        "expected routed-root parse failure message: {result:#?}"
    );
    assert!(
        result
            .message()
            .contains("while resolving clippy coverage and policy roots"),
        "expected coverage/root-resolution context: {result:#?}"
    );
    assert!(!result.inventory());
}
