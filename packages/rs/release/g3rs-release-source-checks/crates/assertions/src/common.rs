use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Comparable view of a single check finding for tests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    /// Severity reported by the rule.
    severity: G3Severity,
    /// Short title text.
    title: &'a str,
    /// Long message text.
    message: &'a str,
    /// Optional file the finding points at.
    file: Option<&'a str>,
    /// True when the finding is inventory-only rather than a violation.
    inventory: bool,
}

/// Stable sort key for a finding.
type FindingSortKey<'a> = (String, &'a str, &'a str, Option<&'a str>, bool);

/// Builds a stable sort key for `finding`.
fn sort_key<'a>(finding: &Finding<'a>) -> FindingSortKey<'a> {
    (
        format!("{:?}", finding.severity),
        finding.title,
        finding.message,
        finding.file,
        finding.inventory,
    )
}

/// Returns the sorted findings emitted by the rule with `id`.
#[must_use]
pub(crate) fn findings<'a>(results: &'a [G3CheckResult], id: &str) -> Vec<Finding<'a>> {
    let mut findings = results
        .iter()
        .filter(|result| result.id() == id)
        .map(|result| Finding {
            severity: result.severity(),
            title: result.title(),
            message: result.message(),
            file: result.file(),
            inventory: result.inventory(),
        })
        .collect::<Vec<_>>();
    findings.sort_by(|left, right| sort_key(left).cmp(&sort_key(right)));
    findings
}

/// Asserts the rule with `id` produced exactly `expected` (order-insensitive).
pub(crate) fn assert_findings(results: &[G3CheckResult], id: &str, expected: &[Finding<'_>]) {
    let mut expected_vec = expected.to_vec();
    expected_vec.sort_by(|left, right| sort_key(left).cmp(&sort_key(right)));
    assert_eq!(
        findings(results, id),
        expected_vec,
        "findings mismatch for `{id}`",
    );
}

/// Asserts the rule with `id` produced no findings.
pub(crate) fn assert_no_findings(results: &[G3CheckResult], id: &str) {
    assert!(
        findings(results, id).is_empty(),
        "expected no findings for `{id}`",
    );
}

/// Constructs a `Finding` with a concrete file path.
#[must_use]
pub(crate) const fn finding<'a>(
    severity: G3Severity,
    title: &'a str,
    message: &'a str,
    file: &'a str,
    inventory: bool,
) -> Finding<'a> {
    Finding {
        severity,
        title,
        message,
        file: Some(file),
        inventory,
    }
}

#[macro_export]
macro_rules! define_result_assertions {
    ($id:literal) => {
        pub use $crate::common::Finding;

        pub fn assert_findings(
            results: &[guardrail3_check_types::G3CheckResult],
            expected: &[Finding<'_>],
        ) {
            $crate::common::assert_findings(results, $id, expected);
        }

        pub fn assert_no_findings(results: &[guardrail3_check_types::G3CheckResult]) {
            $crate::common::assert_no_findings(results, $id);
        }

        #[must_use]
        pub const fn error<'a>(
            title: &'a str,
            message: &'a str,
            file: &'a str,
            inventory: bool,
        ) -> Finding<'a> {
            $crate::common::finding(
                guardrail3_check_types::G3Severity::Error,
                title,
                message,
                file,
                inventory,
            )
        }

        #[must_use]
        pub const fn info<'a>(
            title: &'a str,
            message: &'a str,
            file: &'a str,
            inventory: bool,
        ) -> Finding<'a> {
            $crate::common::finding(
                guardrail3_check_types::G3Severity::Info,
                title,
                message,
                file,
                inventory,
            )
        }
    };
}
