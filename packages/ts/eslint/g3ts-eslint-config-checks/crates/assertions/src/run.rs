use guardrail3_check_types::{G3CheckResult, G3Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    pub id: &'a str,
    pub severity: G3Severity,
    pub title: &'a str,
    pub message: &'a str,
    pub file: Option<&'a str>,
    pub inventory: bool,
}

/// Fails the calling test when `results` does not contain every finding in `expected`.
///
/// # Panics
/// Panics on missing expected finding, which the assertion treats as a test failure.
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

/// Fails the calling test when `results` does not exactly match `expected` in content and order.
///
/// # Panics
/// Panics on mismatch, which the assertion treats as a test failure.
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

/// Fails the calling test when `results` ids do not match `expected` exactly and in order.
///
/// # Panics
/// Panics on id mismatch, which the assertion treats as a test failure.
pub fn assert_exact_ids(results: &[G3CheckResult], expected: &[&str]) {
    let actual = results.iter().map(G3CheckResult::id).collect::<Vec<_>>();
    assert_eq!(actual, expected, "exact finding id order mismatch");
}

/// Fails the calling test when `results` contains any finding with `id`.
///
/// # Panics
/// Panics when matching findings are present, which the assertion treats as a test failure.
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
