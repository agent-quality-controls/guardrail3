use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Expected check result fields used by per-rule assertion helpers in this crate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    /// Severity emitted by the check rule.
    severity: G3Severity,
    /// Human-readable title emitted by the check rule.
    title: &'a str,
    /// Human-readable message emitted by the check rule.
    message: &'a str,
    /// File path the finding refers to, when set.
    file: Option<&'a str>,
    /// Whether the finding is an inventory record rather than a violation.
    inventory: bool,
}

/// Returns the deterministic, sorted list of findings emitted by the rule with `id`.
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
    findings.sort_by(|left, right| {
        (
            format!("{:?}", left.severity),
            left.title,
            left.message,
            left.file,
            left.inventory,
        )
            .cmp(&(
                format!("{:?}", right.severity),
                right.title,
                right.message,
                right.file,
                right.inventory,
            ))
    });
    findings
}

/// Asserts that the rule with `id` emitted at least one finding equal to `expected`.
///
/// # Panics
///
/// Panics when no matching finding exists in `results`.
pub(crate) fn assert_contains(results: &[G3CheckResult], id: &str, expected: &Finding<'_>) {
    let actual = findings(results, id);
    assert!(
        actual.contains(expected),
        "expected {id} finding {expected:#?} not present in {actual:#?}"
    );
}

/// Constructs a `Finding` using the canonical field order used by per-rule assertions.
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

/// Asserts that the rule with `rule_id` emitted no findings.
///
/// # Panics
///
/// Panics when at least one finding for `rule_id` exists in `results`.
pub(crate) fn assert_rule_quiet(results: &[G3CheckResult], rule_id: &str) {
    let actual = findings(results, rule_id);
    assert!(
        actual.is_empty(),
        "expected no {rule_id} findings, got {actual:#?}"
    );
}

#[macro_export]
macro_rules! define_result_assertions {
    ($rule_id:literal) => {
        pub use $crate::common::Finding;

        #[must_use]
        pub fn findings(results: &[guardrail3_check_types::G3CheckResult]) -> Vec<Finding<'_>> {
            $crate::common::findings(results, $rule_id)
        }

        pub fn assert_contains(
            results: &[guardrail3_check_types::G3CheckResult],
            expected: Finding<'_>,
        ) {
            $crate::common::assert_contains(results, $rule_id, &expected);
        }

        pub fn assert_rule_quiet(results: &[guardrail3_check_types::G3CheckResult]) {
            $crate::common::assert_rule_quiet(results, $rule_id);
        }

        pub fn assert_no_findings(results: &[guardrail3_check_types::G3CheckResult]) {
            $crate::common::assert_rule_quiet(results, $rule_id);
        }

        #[must_use]
        pub const fn error<'a>(title: &'a str, message: &'a str, file: &'a str) -> Finding<'a> {
            $crate::common::finding(
                guardrail3_check_types::G3Severity::Error,
                title,
                message,
                file,
                false,
            )
        }

        #[must_use]
        pub const fn inventory<'a>(title: &'a str, message: &'a str, file: &'a str) -> Finding<'a> {
            $crate::common::finding(
                guardrail3_check_types::G3Severity::Error,
                title,
                message,
                file,
                true,
            )
        }
    };
}
