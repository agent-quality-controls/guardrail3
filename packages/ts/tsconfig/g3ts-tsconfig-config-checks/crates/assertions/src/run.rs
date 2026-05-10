use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Test-side finding shape used to assert against `G3CheckResult` instances.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    /// Finding ID matching the rule's `G3CheckResult::id`.
    id: &'a str,
    /// Finding severity.
    severity: G3Severity,
    /// Short finding title.
    title: &'a str,
    /// Detailed finding message.
    message: &'a str,
    /// Optional workspace-relative path associated with the finding.
    file: Option<&'a str>,
    /// Whether the finding is an inventory entry.
    inventory: bool,
}

/// Assert that every entry in `expected` is present in `results`.
///
/// # Panics
///
/// Panics when any entry in `expected` is missing from `results`.
pub fn assert_contains(results: &[G3CheckResult], expected: &[Finding<'_>]) {
    let actual = results
        .iter()
        .map(|result| Finding {
            id: result.id(),
            severity: result.severity(),
            title: result.title(),
            message: result.message(),
            file: result.file(),
            inventory: result.inventory(),
        })
        .collect::<Vec<_>>();

    for expected_finding in expected {
        assert!(
            actual.contains(expected_finding),
            "expected finding {expected_finding:?}, got: {actual:?}",
        );
    }
}

/// Assert that `results` (after projection to `Finding`) equals `expected`
/// exactly, in order.
///
/// # Panics
///
/// Panics when the projected results differ from `expected`.
pub fn assert_exact(results: &[G3CheckResult], expected: &[Finding<'_>]) {
    let actual = results
        .iter()
        .map(|result| Finding {
            id: result.id(),
            severity: result.severity(),
            title: result.title(),
            message: result.message(),
            file: result.file(),
            inventory: result.inventory(),
        })
        .collect::<Vec<_>>();

    assert_eq!(actual, expected, "exact findings mismatch");
}

/// Assert that the IDs of `results` exactly equal `expected`, in order.
///
/// # Panics
///
/// Panics when the result IDs differ from `expected`.
pub fn assert_exact_ids(results: &[G3CheckResult], expected: &[&str]) {
    let actual = results.iter().map(G3CheckResult::id).collect::<Vec<_>>();
    assert_eq!(actual, expected, "exact finding id order mismatch");
}

/// Assert that no finding in `results` carries `id`.
///
/// # Panics
///
/// Panics when at least one finding with `id` is present in `results`.
pub fn assert_no_findings_for_id(results: &[G3CheckResult], id: &str) {
    let matching = results
        .iter()
        .filter(|result| result.id() == id)
        .collect::<Vec<_>>();
    assert!(
        matching.is_empty(),
        "expected no findings for `{id}`, got: {matching:?}",
    );
}

#[must_use]
pub const fn error<'a>(
    id: &'a str,
    title: &'a str,
    message: &'a str,
    file: Option<&'a str>,
    inventory: bool,
) -> Finding<'a> {
    Finding {
        id,
        severity: G3Severity::Error,
        title,
        message,
        file,
        inventory,
    }
}

#[must_use]
pub const fn info<'a>(
    id: &'a str,
    title: &'a str,
    message: &'a str,
    file: Option<&'a str>,
    inventory: bool,
) -> Finding<'a> {
    Finding {
        id,
        severity: G3Severity::Info,
        title,
        message,
        file,
        inventory,
    }
}
