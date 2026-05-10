use guardrail3_check_types::{G3CheckResult, G3Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    /// `severity` field.
    severity: G3Severity,
    /// `title` field.
    title: &'a str,
    /// `message` field.
    message: &'a str,
    /// `file` field.
    file: Option<&'a str>,
    /// `inventory` field.
    inventory: bool,
}

/// Collect findings produced for the given rule id, sorted for stable comparisons.
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

/// Assert that the rule produced a finding matching `expected`.
pub(crate) fn assert_contains(results: &[G3CheckResult], id: &str, expected: &Finding<'_>) {
    let actual = findings(results, id);
    assert!(
        actual.contains(expected),
        "expected {id} finding {expected:#?} not found in {actual:#?}"
    );
}

/// Build a `Finding` value for comparison in tests.
#[must_use]
pub(crate) const fn finding<'a>(
    severity: G3Severity,
    title: &'a str,
    message: &'a str,
    file: Option<&'a str>,
    inventory: bool,
) -> Finding<'a> {
    Finding {
        severity,
        title,
        message,
        file,
        inventory,
    }
}

/// Assert that the rule produced no findings.
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
        pub use guardrail3_check_types::G3Severity as Severity;
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
        pub const fn finding<'a>(
            severity: Severity,
            title: &'a str,
            message: &'a str,
            file: Option<&'a str>,
            inventory: bool,
        ) -> Finding<'a> {
            $crate::common::finding(severity, title, message, file, inventory)
        }
    };
}
