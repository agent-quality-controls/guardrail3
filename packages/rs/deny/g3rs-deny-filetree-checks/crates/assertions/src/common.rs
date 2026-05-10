use guardrail3_check_types::{G3CheckResult, G3Severity};

/// One expected check finding used for assertion-driven snapshot comparison.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    /// Severity expected for the finding.
    severity: G3Severity,
    /// Expected human-readable title of the finding.
    title: &'a str,
    /// Expected human-readable message body of the finding.
    message: &'a str,
    /// Optional workspace-relative file path expected on the finding.
    file: Option<&'a str>,
    /// Whether the finding is expected to be filtered as inventory.
    inventory: bool,
}

/// Sort key shape used to compare findings deterministically.
type FindingKey<'a> = (String, &'a str, &'a str, Option<&'a str>, bool);

/// Returns a stable comparison key for the given finding.
fn finding_key<'a>(finding: &Finding<'a>) -> FindingKey<'a> {
    (
        format!("{:?}", finding.severity),
        finding.title,
        finding.message,
        finding.file,
        finding.inventory,
    )
}

/// Sorts findings into the deterministic snapshot comparison order.
fn sort_findings(findings: &mut [Finding<'_>]) {
    findings.sort_by(|left, right| finding_key(left).cmp(&finding_key(right)));
}

/// Returns the findings emitted under `id`, sorted into a stable comparison order.
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
    sort_findings(&mut findings);
    findings
}

/// Asserts that the findings emitted under `id` match `expected` after sorting.
pub(crate) fn assert_findings(results: &[G3CheckResult], id: &str, expected: &[Finding<'_>]) {
    let mut expected_vec = expected.to_vec();
    sort_findings(&mut expected_vec);
    assert_eq!(
        findings(results, id),
        expected_vec,
        "deny findings under id `{id}` did not match the expected snapshot",
    );
}

/// Asserts that no findings were emitted under `id`.
pub(crate) fn assert_no_findings(results: &[G3CheckResult], id: &str) {
    assert!(
        findings(results, id).is_empty(),
        "deny findings under id `{id}` were expected to be empty",
    );
}

/// Constructs one expected [`Finding`] from its primitive fields.
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

#[macro_export]
macro_rules! define_result_assertions {
    ($id:literal) => {
        pub use $crate::common::Finding;

        #[must_use]
        pub fn findings(results: &[guardrail3_check_types::G3CheckResult]) -> Vec<Finding<'_>> {
            $crate::common::findings(results, $id)
        }

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
                Some(file),
                inventory,
            )
        }

        #[must_use]
        pub const fn error_no_file<'a>(
            title: &'a str,
            message: &'a str,
            inventory: bool,
        ) -> Finding<'a> {
            $crate::common::finding(
                guardrail3_check_types::G3Severity::Error,
                title,
                message,
                None,
                inventory,
            )
        }

        #[must_use]
        pub const fn warn<'a>(
            title: &'a str,
            message: &'a str,
            file: &'a str,
            inventory: bool,
        ) -> Finding<'a> {
            $crate::common::finding(
                guardrail3_check_types::G3Severity::Warn,
                title,
                message,
                Some(file),
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
                Some(file),
                inventory,
            )
        }
    };
}
