use guardrail3_check_types::{G3CheckResult, G3Severity};

/// One expected finding to compare against runtime results.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    /// Severity of the finding.
    severity: G3Severity,
    /// Title text.
    title: &'a str,
    /// Message body.
    message: &'a str,
    /// Optional file rel-path the finding points at.
    file: Option<&'a str>,
    /// Whether the finding is an inventory entry rather than a violation.
    inventory: bool,
}

/// Stable sort key tuple type for `Finding`.
type FindingSortKey<'a> = (String, &'a str, &'a str, Option<&'a str>, bool);

/// Stable sort key for `Finding`, normalizing severity to a string for ordering.
fn sort_key<'a>(finding: &Finding<'a>) -> FindingSortKey<'a> {
    (
        format!("{:?}", finding.severity),
        finding.title,
        finding.message,
        finding.file,
        finding.inventory,
    )
}

/// Collect findings with rule id `id` from `results`, sorted into a stable order for assertions.
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

/// Assert that `expected` is one of the findings for rule id `id`.
pub(crate) fn assert_contains(results: &[G3CheckResult], id: &str, expected: &Finding<'_>) {
    let actual = findings(results, id);
    assert!(
        actual.contains(expected),
        "expected finding for rule `{id}` not present in {actual:#?}"
    );
}

/// Construct a `Finding` for use in assertions.
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

/// Define per-rule assertion helpers (`findings`, `assert_contains`, `error`, `warn`, `info`).
#[macro_export]
macro_rules! define_result_assertions {
    ($id:literal) => {
        pub use $crate::common::Finding;

        /// Return all findings produced by the rule under test.
        #[must_use]
        pub fn findings(results: &[guardrail3_check_types::G3CheckResult]) -> Vec<Finding<'_>> {
            $crate::common::findings(results, $id)
        }

        /// Assert `expected` is among the findings produced by the rule under test.
        ///
        /// # Panics
        ///
        /// Panics when `expected` is not present in the produced findings.
        pub fn assert_contains(
            results: &[guardrail3_check_types::G3CheckResult],
            expected: &Finding<'_>,
        ) {
            $crate::common::assert_contains(results, $id, expected);
        }

        /// Construct an error-severity expected finding.
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

        /// Construct a warn-severity expected finding.
        #[must_use]
        pub const fn warn<'a>(title: &'a str, message: &'a str, file: &'a str) -> Finding<'a> {
            $crate::common::finding(
                guardrail3_check_types::G3Severity::Warn,
                title,
                message,
                file,
                false,
            )
        }

        /// Construct an info-severity expected finding.
        #[must_use]
        pub const fn info<'a>(title: &'a str, message: &'a str, file: &'a str) -> Finding<'a> {
            $crate::common::finding(
                guardrail3_check_types::G3Severity::Info,
                title,
                message,
                file,
                true,
            )
        }
    };
}
