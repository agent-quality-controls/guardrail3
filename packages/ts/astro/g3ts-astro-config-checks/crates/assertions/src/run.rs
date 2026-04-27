use guardrail3_check_types::{G3CheckResult, G3Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    id: &'a str,
    severity: G3Severity,
    title: &'a str,
    message: &'a str,
    file: Option<&'a str>,
    inventory: bool,
}

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

pub fn assert_exact_ids(results: &[G3CheckResult], expected: &[&str]) {
    let actual = results.iter().map(G3CheckResult::id).collect::<Vec<_>>();
    assert_eq!(actual, expected, "exact finding id order mismatch");
}

pub fn assert_all_inventory(results: &[G3CheckResult]) {
    assert!(
        results.iter().all(G3CheckResult::inventory),
        "expected all findings to be inventory, got: {results:?}",
    );
}

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

pub fn assert_has_error_title(results: &[G3CheckResult], id: &str, title: &str) {
    assert_has_id_title_severity(results, id, title, G3Severity::Error);
}

pub fn assert_has_info_title(results: &[G3CheckResult], id: &str, title: &str) {
    assert_has_id_title_severity(results, id, title, G3Severity::Info);
}

pub fn assert_id_message_contains(results: &[G3CheckResult], id: &str, expected: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.id() == id && result.message().contains(expected)),
        "expected finding {id} message to contain {expected:?}, got: {results:?}"
    );
}

pub fn assert_no_id_message_contains(results: &[G3CheckResult], id: &str, expected: &str) {
    assert!(
        results
            .iter()
            .all(|result| result.id() != id || !result.message().contains(expected)),
        "expected no finding {id} message to contain {expected:?}, got: {results:?}"
    );
}

fn assert_has_id_title_severity(
    results: &[G3CheckResult],
    id: &str,
    title: &str,
    severity: G3Severity,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == id && result.title() == title && result.severity() == severity
        }),
        "expected {severity:?} {id} / {title}, got: {results:?}"
    );
}

#[must_use]
pub fn error<'a>(
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
pub fn info<'a>(
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
