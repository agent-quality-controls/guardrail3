use guardrail3_check_types::{G3CheckResult, G3Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    severity: G3Severity,
    title: &'a str,
    message: &'a str,
    file: Option<&'a str>,
    inventory: bool,
}

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

pub(crate) fn assert_contains(results: &[G3CheckResult], id: &str, expected: Finding<'_>) {
    assert!(findings(results, id).contains(&expected));
}

#[must_use]
pub(crate) fn finding<'a>(
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
        pub use crate::common::Finding;

        #[must_use]
        pub fn findings(results: &[guardrail3_check_types::G3CheckResult]) -> Vec<Finding<'_>> {
            crate::common::findings(results, $id)
        }

        pub fn assert_contains(
            results: &[guardrail3_check_types::G3CheckResult],
            expected: Finding<'_>,
        ) {
            crate::common::assert_contains(results, $id, expected);
        }

        #[must_use]
        pub fn error<'a>(title: &'a str, message: &'a str, file: &'a str) -> Finding<'a> {
            crate::common::finding(
                guardrail3_check_types::G3Severity::Error,
                title,
                message,
                file,
                false,
            )
        }

        #[must_use]
        pub fn warn<'a>(title: &'a str, message: &'a str, file: &'a str) -> Finding<'a> {
            crate::common::finding(
                guardrail3_check_types::G3Severity::Warn,
                title,
                message,
                file,
                false,
            )
        }

        #[must_use]
        pub fn info<'a>(title: &'a str, message: &'a str, file: &'a str) -> Finding<'a> {
            crate::common::finding(
                guardrail3_check_types::G3Severity::Info,
                title,
                message,
                file,
                true,
            )
        }
    };
}
