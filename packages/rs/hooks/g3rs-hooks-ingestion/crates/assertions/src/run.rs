use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Decoded finding shape used by assertion helpers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    /// Severity reported by the rule.
    severity: G3Severity,
    /// Short title.
    title: &'a str,
    /// Full message body.
    message: &'a str,
    /// Workspace-relative file path if present.
    file: Option<&'a str>,
    /// 1-based line number if present.
    line: Option<usize>,
    /// Whether the finding is inventory-only.
    inventory: bool,
}

#[must_use]
pub fn findings<'a>(results: &'a [G3CheckResult], id: &str) -> Vec<Finding<'a>> {
    let mut findings = results
        .iter()
        .filter(|result| result.id() == id)
        .map(Finding::from_result)
        .collect::<Vec<_>>();
    findings.sort_by(|left, right| {
        (
            format!("{:?}", left.severity),
            left.title,
            left.message,
            left.file,
            left.line,
            left.inventory,
        )
            .cmp(&(
                format!("{:?}", right.severity),
                right.title,
                right.message,
                right.file,
                right.line,
                right.inventory,
            ))
    });
    findings
}

#[must_use]
pub fn non_inventory(results: &[G3CheckResult]) -> Vec<Finding<'_>> {
    let mut findings = results
        .iter()
        .filter(|result| !result.inventory())
        .map(Finding::from_result)
        .collect::<Vec<_>>();
    findings.sort_by(|left, right| {
        (
            format!("{:?}", left.severity),
            left.title,
            left.message,
            left.file,
            left.line,
            left.inventory,
        )
            .cmp(&(
                format!("{:?}", right.severity),
                right.title,
                right.message,
                right.file,
                right.line,
                right.inventory,
            ))
    });
    findings
}

#[must_use]
pub fn count(results: &[G3CheckResult], id: &str) -> usize {
    findings(results, id).len()
}

/// Assert at least one finding for `id` matches title/file/inventory.
///
/// # Panics
/// Panics when no matching finding exists.
pub fn assert_present(
    results: &[G3CheckResult],
    id: &str,
    title: &str,
    file: Option<&str>,
    inventory: bool,
) {
    let findings = findings(results, id);
    assert!(
        findings.iter().any(|finding| finding.title == title
            && finding.file == file
            && finding.inventory == inventory),
        "{findings:#?}"
    );
}

/// Assert at least one finding matches and its message contains `needle`.
///
/// # Panics
/// Panics when no matching finding exists.
pub fn assert_message_contains(
    results: &[G3CheckResult],
    id: &str,
    title: &str,
    file: Option<&str>,
    inventory: bool,
    needle: &str,
) {
    let findings = findings(results, id);
    assert!(
        findings.iter().any(|finding| {
            finding.title == title
                && finding.file == file
                && finding.inventory == inventory
                && finding.message.contains(needle)
        }),
        "{findings:#?}"
    );
}

/// Assert at least one finding matches and its message equals `message`.
///
/// # Panics
/// Panics when no matching finding exists.
pub fn assert_message_equals(
    results: &[G3CheckResult],
    id: &str,
    title: &str,
    file: Option<&str>,
    inventory: bool,
    message: &str,
) {
    let findings = findings(results, id);
    assert!(
        findings.iter().any(|finding| {
            finding.title == title
                && finding.file == file
                && finding.inventory == inventory
                && finding.message == message
        }),
        "{findings:#?}"
    );
}

/// Assert at least one finding matches at the given 1-based `line`.
///
/// # Panics
/// Panics when no matching finding exists.
pub fn assert_present_on_line(
    results: &[G3CheckResult],
    id: &str,
    title: &str,
    file: Option<&str>,
    inventory: bool,
    line: usize,
) {
    let findings = findings(results, id);
    assert!(
        findings.iter().any(|finding| {
            finding.title == title
                && finding.file == file
                && finding.inventory == inventory
                && finding.line == Some(line)
        }),
        "{findings:#?}"
    );
}

/// Assert no findings exist for the given `file`.
///
/// # Panics
/// Panics when any finding references `file`.
pub fn assert_no_results_for_file(results: &[G3CheckResult], file: &str) {
    let findings = results
        .iter()
        .filter(|result| result.file() == Some(file))
        .map(Finding::from_result)
        .collect::<Vec<_>>();
    assert!(findings.is_empty(), "{findings:#?}");
}

impl<'a> Finding<'a> {
    /// Build a Finding view from a `G3CheckResult`.
    fn from_result(result: &'a G3CheckResult) -> Self {
        Self {
            severity: result.severity(),
            title: result.title(),
            message: result.message(),
            file: result.file(),
            line: result.line(),
            inventory: result.inventory(),
        }
    }
}
