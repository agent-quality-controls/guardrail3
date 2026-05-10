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

#[must_use]
/// `findings` function.
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

/// `assert_findings` function.
pub(crate) fn assert_findings(results: &[G3CheckResult], id: &str, expected: &[Finding<'_>]) {
    let mut expected_vec = expected.to_vec();
    expected_vec.sort_by(|left, right| {
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
    assert_eq!(
        findings(results, id),
        expected_vec,
        "findings for id `{id}` did not match expected set",
    );
}

/// `assert_no_findings` function.
pub(crate) fn assert_no_findings(results: &[G3CheckResult], id: &str) {
    assert!(
        findings(results, id).is_empty(),
        "expected no findings for id `{id}`",
    );
}

/// `assert_contains` function.
pub(crate) fn assert_contains(results: &[G3CheckResult], id: &str, expected: &[Finding<'_>]) {
    let findings = findings(results, id);
    for expected_finding in expected {
        assert!(
            findings.contains(expected_finding),
            "expected finding {expected_finding:?}, got: {findings:?}",
        );
    }
}

#[must_use]
/// `count_titles_with_prefix` function.
pub(crate) fn count_titles_with_prefix(results: &[G3CheckResult], id: &str, prefix: &str) -> usize {
    findings(results, id)
        .iter()
        .filter(|finding| finding.title.starts_with(prefix))
        .count()
}

#[must_use]
/// `fn` function.
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

        pub fn assert_contains(
            results: &[guardrail3_check_types::G3CheckResult],
            expected: &[Finding<'_>],
        ) {
            $crate::common::assert_contains(results, $id, expected);
        }

        pub fn assert_title_count(
            results: &[guardrail3_check_types::G3CheckResult],
            prefix: &str,
            expected_count: usize,
        ) {
            assert_eq!(
                $crate::common::count_titles_with_prefix(results, $id, prefix),
                expected_count,
                "unexpected title count for prefix `{prefix}`",
            );
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
