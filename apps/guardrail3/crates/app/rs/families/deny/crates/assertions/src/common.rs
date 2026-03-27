use guardrail3_domain_report::{CheckResult, Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    pub severity: Severity,
    pub title: &'a str,
    pub message: &'a str,
    pub file: Option<&'a str>,
    pub inventory: bool,
}

#[must_use]
pub fn findings<'a>(results: &'a [CheckResult], id: &str) -> Vec<Finding<'a>> {
    let mut findings = results
        .iter()
        .filter(|result| result.id == id)
        .map(|result| Finding {
            severity: result.severity,
            title: result.title.as_str(),
            message: result.message.as_str(),
            file: result.file.as_deref(),
            inventory: result.inventory,
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

pub fn assert_findings(results: &[CheckResult], id: &str, expected: &[Finding<'_>]) {
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

pub fn assert_no_findings(results: &[CheckResult], id: &str) {
    assert!(findings(results, id).is_empty());
}

pub fn assert_contains(results: &[CheckResult], id: &str, expected: Finding<'_>) {
    assert!(findings(results, id).contains(&expected));
}

#[must_use]
pub fn finding<'a>(
    severity: Severity,
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
        pub use crate::common::Finding;

        #[must_use]
        pub fn findings(
            results: &[guardrail3_domain_report::CheckResult],
        ) -> Vec<Finding<'_>> {
            crate::common::findings(results, $id)
        }

        pub fn assert_findings(
            results: &[guardrail3_domain_report::CheckResult],
            expected: &[Finding<'_>],
        ) {
            crate::common::assert_findings(results, $id, expected);
        }

        pub fn assert_no_findings(results: &[guardrail3_domain_report::CheckResult]) {
            crate::common::assert_no_findings(results, $id);
        }

        pub fn assert_contains(
            results: &[guardrail3_domain_report::CheckResult],
            expected: Finding<'_>,
        ) {
            crate::common::assert_contains(results, $id, expected);
        }

        #[must_use]
        pub fn error<'a>(
            title: &'a str,
            message: &'a str,
            file: &'a str,
            inventory: bool,
        ) -> Finding<'a> {
            crate::common::finding(
                guardrail3_domain_report::Severity::Error,
                title,
                message,
                Some(file),
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
                guardrail3_domain_report::Severity::Warn,
                title,
                message,
                Some(file),
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
                guardrail3_domain_report::Severity::Info,
                title,
                message,
                Some(file),
                inventory,
            )
        }

        #[must_use]
        pub fn error_no_file<'a>(
            title: &'a str,
            message: &'a str,
            inventory: bool,
        ) -> Finding<'a> {
            crate::common::finding(
                guardrail3_domain_report::Severity::Error,
                title,
                message,
                None,
                inventory,
            )
        }

        #[must_use]
        pub fn warn_no_file<'a>(
            title: &'a str,
            message: &'a str,
            inventory: bool,
        ) -> Finding<'a> {
            crate::common::finding(
                guardrail3_domain_report::Severity::Warn,
                title,
                message,
                None,
                inventory,
            )
        }

        #[must_use]
        pub fn info_no_file<'a>(
            title: &'a str,
            message: &'a str,
            inventory: bool,
        ) -> Finding<'a> {
            crate::common::finding(
                guardrail3_domain_report::Severity::Info,
                title,
                message,
                None,
                inventory,
            )
        }
    };
}
