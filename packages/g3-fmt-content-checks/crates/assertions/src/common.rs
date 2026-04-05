use guardrail3_check_types::{GrdzCheckResult, GrdzSeverity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    pub severity: GrdzSeverity,
    pub title: &'a str,
    pub message: &'a str,
    pub file: Option<&'a str>,
    pub inventory: bool,
}

#[must_use]
pub(crate) fn findings<'a>(results: &'a [GrdzCheckResult], id: &str) -> Vec<Finding<'a>> {
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

pub(crate) fn assert_findings(
    results: &[GrdzCheckResult],
    id: &str,
    expected: &[Finding<'_>],
) {
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
    assert_eq!(findings(results, id), expected_vec);
}

pub(crate) fn assert_no_findings(results: &[GrdzCheckResult], id: &str) {
    assert!(findings(results, id).is_empty());
}

#[must_use]
pub(crate) fn finding<'a>(
    severity: GrdzSeverity,
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
        pub fn findings(results: &[guardrail3_check_types::GrdzCheckResult]) -> Vec<Finding<'_>> {
            crate::common::findings(results, $id)
        }

        pub fn assert_findings(
            results: &[guardrail3_check_types::GrdzCheckResult],
            expected: &[Finding<'_>],
        ) {
            crate::common::assert_findings(results, $id, expected);
        }

        pub fn assert_no_findings(results: &[guardrail3_check_types::GrdzCheckResult]) {
            crate::common::assert_no_findings(results, $id);
        }

        #[must_use]
        pub fn error<'a>(
            title: &'a str,
            message: &'a str,
            file: &'a str,
            inventory: bool,
        ) -> Finding<'a> {
            crate::common::finding(
                guardrail3_check_types::GrdzSeverity::Error,
                title,
                message,
                file,
                inventory,
            )
        }

        #[must_use]
        pub fn warn<'a>(
            title: &'a str,
            message: &'a str,
            file: &'a str,
            inventory: bool,
        ) -> Finding<'a> {
            crate::common::finding(
                guardrail3_check_types::GrdzSeverity::Warn,
                title,
                message,
                file,
                inventory,
            )
        }

        #[must_use]
        pub fn info<'a>(
            title: &'a str,
            message: &'a str,
            file: &'a str,
            inventory: bool,
        ) -> Finding<'a> {
            crate::common::finding(
                guardrail3_check_types::GrdzSeverity::Info,
                title,
                message,
                file,
                inventory,
            )
        }
    };
}
