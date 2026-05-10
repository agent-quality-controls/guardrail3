use guardrail3_check_types::{G3CheckResult, G3Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    /// Field `severity`.
    severity: G3Severity,
    /// Field `title`.
    title: &'a str,
    /// Field `message`.
    message: &'a str,
    /// Field `file`.
    file: Option<&'a str>,
    /// Field `inventory`.
    inventory: bool,
}

#[must_use]
/// Implements `findings`.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
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

/// Asserts the `findings` invariant on `results`.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
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
        "mismatched findings for `{id}`"
    );
}

/// Asserts the `no findings` invariant on `results`.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
pub(crate) fn assert_no_findings(results: &[G3CheckResult], id: &str) {
    assert!(
        findings(results, id).is_empty(),
        "{:#?}",
        findings(results, id)
    );
}

/// Asserts the `has finding` invariant on `results`.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
pub(crate) fn assert_has_finding(
    results: &[G3CheckResult],
    id: &str,
    expected_severity: G3Severity,
    expected_title: &str,
    expected_inventory: bool,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == id
                && result.severity() == expected_severity
                && result.title() == expected_title
                && result.inventory() == expected_inventory
        }),
        "{:#?}",
        findings(results, id)
    );
}

/// Asserts the `title count` invariant on `results`.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
pub(crate) fn assert_title_count(
    results: &[G3CheckResult],
    id: &str,
    expected_title: &str,
    expected_count: usize,
) {
    let actual = results
        .iter()
        .filter(|result| result.id() == id && result.title() == expected_title)
        .count();
    assert_eq!(actual, expected_count, "{:#?}", findings(results, id));
}

/// Asserts the `message contains` invariant on `results`.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
pub(crate) fn assert_message_contains(
    results: &[G3CheckResult],
    id: &str,
    expected_title: &str,
    needle: &str,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == id
                && result.title() == expected_title
                && result.message().contains(needle)
        }),
        "{:#?}",
        findings(results, id)
    );
}

/// Asserts the `title absent` invariant on `results`.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
pub(crate) fn assert_title_absent(results: &[G3CheckResult], id: &str, expected_title: &str) {
    assert!(
        results
            .iter()
            .all(|result| !(result.id() == id && result.title() == expected_title)),
        "{:#?}",
        findings(results, id)
    );
}

#[must_use]
/// Implements `finding`.
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

        pub fn assert_has_info(
            results: &[guardrail3_check_types::G3CheckResult],
            title: &str,
            inventory: bool,
        ) {
            $crate::common::assert_has_finding(
                results,
                $id,
                guardrail3_check_types::G3Severity::Info,
                title,
                inventory,
            );
        }

        pub fn assert_has_warn(
            results: &[guardrail3_check_types::G3CheckResult],
            title: &str,
            inventory: bool,
        ) {
            $crate::common::assert_has_finding(
                results,
                $id,
                guardrail3_check_types::G3Severity::Warn,
                title,
                inventory,
            );
        }

        pub fn assert_has_error(
            results: &[guardrail3_check_types::G3CheckResult],
            title: &str,
            inventory: bool,
        ) {
            $crate::common::assert_has_finding(
                results,
                $id,
                guardrail3_check_types::G3Severity::Error,
                title,
                inventory,
            );
        }

        pub fn assert_title_count(
            results: &[guardrail3_check_types::G3CheckResult],
            title: &str,
            expected_count: usize,
        ) {
            $crate::common::assert_title_count(results, $id, title, expected_count);
        }

        pub fn assert_message_contains(
            results: &[guardrail3_check_types::G3CheckResult],
            title: &str,
            needle: &str,
        ) {
            $crate::common::assert_message_contains(results, $id, title, needle);
        }

        pub fn assert_title_absent(results: &[guardrail3_check_types::G3CheckResult], title: &str) {
            $crate::common::assert_title_absent(results, $id, title);
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
