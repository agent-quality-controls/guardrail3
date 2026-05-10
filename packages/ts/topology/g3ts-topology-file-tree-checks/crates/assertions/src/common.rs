use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Borrowed projection of a [`G3CheckResult`] used in test assertions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    /// Severity reported by the rule.
    severity: G3Severity,
    /// Rule title surfaced to users.
    title: &'a str,
    /// Human-readable rule message.
    message: &'a str,
    /// Optional unit-relative file path.
    file: Option<&'a str>,
    /// Whether the result is an inventory finding rather than a violation.
    inventory: bool,
}

/// Returns a sorted projection of `results` filtered by rule `id`.
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

/// Asserts that `results` contains a finding for rule `id` matching `expected`.
pub(crate) fn assert_contains(results: &[G3CheckResult], id: &str, expected: &Finding<'_>) {
    assert!(
        findings(results, id).contains(expected),
        "expected finding for {id} to contain {expected:#?}; got {:#?}",
        findings(results, id)
    );
}

/// Constructs a [`Finding`] for use in assertions.
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

/// Asserts that no findings exist for `rule_id` in `results`.
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
            expected: &Finding<'_>,
        ) {
            $crate::common::assert_contains(results, $rule_id, expected);
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
