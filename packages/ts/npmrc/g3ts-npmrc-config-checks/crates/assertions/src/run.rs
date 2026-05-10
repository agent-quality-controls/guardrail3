use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Compact view of a `G3CheckResult` used for finding-equality assertions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    /// Rule identifier.
    id: &'a str,
    /// Severity of the finding.
    severity: G3Severity,
    /// Short result title.
    title: &'a str,
    /// Detailed result message.
    message: &'a str,
    /// File path attached to the result, if any.
    file: Option<&'a str>,
    /// Whether the result is inventory-tagged (informational rather than a
    /// failure).
    inventory: bool,
}

/// Project a `G3CheckResult` into a borrowed `Finding` for comparison.
fn project_result(result: &G3CheckResult) -> Finding<'_> {
    Finding {
        id: result.id(),
        severity: result.severity(),
        title: result.title(),
        message: result.message(),
        file: result.file(),
        inventory: result.inventory(),
    }
}

/// Project every `G3CheckResult` in `results` into a borrowed `Finding` list.
fn project_results(results: &[G3CheckResult]) -> Vec<Finding<'_>> {
    results.iter().map(project_result).collect()
}

/// Assert that every `expected` finding appears in `results`.
///
/// # Panics
///
/// Panics when any expected finding is not present in `results`.
pub fn assert_contains(results: &[G3CheckResult], expected: &[Finding<'_>]) {
    let actual = project_results(results);
    for expected_finding in expected {
        assert!(
            actual.contains(expected_finding),
            "expected finding {expected_finding:?}, got: {actual:?}",
        );
    }
}

/// Assert that `results` matches `expected` exactly, in order.
///
/// # Panics
///
/// Panics when the projected findings differ from `expected`.
pub fn assert_exact(results: &[G3CheckResult], expected: &[Finding<'_>]) {
    let actual = project_results(results);
    assert_eq!(actual, expected, "exact findings mismatch");
}

/// Assert that the rule ids of `results` match `expected` exactly, in order.
///
/// # Panics
///
/// Panics when the result ids differ from `expected`.
pub fn assert_exact_ids(results: &[G3CheckResult], expected: &[&str]) {
    let actual = results.iter().map(G3CheckResult::id).collect::<Vec<_>>();
    assert_eq!(actual, expected, "exact finding id order mismatch");
}

/// Assert that no result in `results` carries the rule `id`.
///
/// # Panics
///
/// Panics when any result has the given `id`.
pub fn assert_no_findings_for_id(results: &[G3CheckResult], id: &str) {
    let matching: Vec<&G3CheckResult> = results.iter().filter(|r| r.id() == id).collect();
    assert!(
        matching.is_empty(),
        "expected no findings for `{id}`, got: {matching:?}",
    );
}

/// Build a `Finding` describing an expected result of the given severity.
#[must_use]
const fn finding_with_severity<'a>(
    severity: G3Severity,
    id: &'a str,
    title: &'a str,
    message: &'a str,
    file: Option<&'a str>,
    inventory: bool,
) -> Finding<'a> {
    Finding {
        id,
        severity,
        title,
        message,
        file,
        inventory,
    }
}

/// Build a `Finding` describing an error-severity expected result.
#[must_use]
pub const fn error<'a>(
    id: &'a str,
    title: &'a str,
    message: &'a str,
    file: Option<&'a str>,
    inventory: bool,
) -> Finding<'a> {
    finding_with_severity(G3Severity::Error, id, title, message, file, inventory)
}

/// Build a `Finding` describing an info-severity expected result.
#[must_use]
pub const fn info<'a>(
    id: &'a str,
    title: &'a str,
    message: &'a str,
    file: Option<&'a str>,
    inventory: bool,
) -> Finding<'a> {
    finding_with_severity(G3Severity::Info, id, title, message, file, inventory)
}
