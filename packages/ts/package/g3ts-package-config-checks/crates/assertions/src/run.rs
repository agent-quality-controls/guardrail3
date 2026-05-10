use guardrail3_check_types::{G3CheckResult, G3Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    /// Stable rule identifier used to group findings.
    id: &'a str,
    /// Severity attached to the finding.
    severity: G3Severity,
    /// Short human-readable title.
    title: &'a str,
    /// Full human-readable message.
    message: &'a str,
    /// Workspace-relative file the finding refers to, if any.
    file: Option<&'a str>,
    /// Whether the finding is informational/inventory only.
    inventory: bool,
}

/// Assert that all expected findings are contained in `results`.
///
/// # Panics
/// Panics if any expected finding is not present.
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

/// Assert that `results` exactly equals `expected` (same order and contents).
///
/// # Panics
/// Panics if `results` and `expected` differ.
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

/// Assert that finding ids in `results` exactly equal `expected`.
///
/// # Panics
/// Panics if the ids differ.
pub fn assert_exact_ids(results: &[G3CheckResult], expected: &[&str]) {
    let actual = results.iter().map(G3CheckResult::id).collect::<Vec<_>>();
    assert_eq!(actual, expected, "exact finding id order mismatch");
}

/// Assert there are no findings with the given id in `results`.
///
/// # Panics
/// Panics if any finding with `id` is present.
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
