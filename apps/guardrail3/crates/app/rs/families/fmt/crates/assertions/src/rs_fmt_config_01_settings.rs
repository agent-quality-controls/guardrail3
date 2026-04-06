use guardrail3_domain_report::{CheckResult, Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    pub severity: Severity,
    pub title: &'a str,
    pub message: &'a str,
    pub file: Option<&'a str>,
    pub inventory: bool,
}

const ID: &str = "RS-FMT-CONFIG-01";

#[must_use]
pub fn findings(results: &[CheckResult]) -> Vec<Finding<'_>> {
    results
        .iter()
        .filter(|result| result.id() == ID)
        .map(|result| Finding {
            severity: result.severity(),
            title: result.title(),
            message: result.message(),
            file: result.file(),
            inventory: result.inventory(),
        })
        .collect()
}

pub fn assert_count(results: &[CheckResult], expected: usize) {
    assert_eq!(findings(results).len(), expected);
}

pub fn assert_contains(results: &[CheckResult], expected: Finding<'_>) {
    assert!(findings(results).contains(&expected));
}

pub fn assert_no_findings(results: &[CheckResult]) {
    assert!(findings(results).is_empty());
}

pub fn assert_parse_error(results: &[CheckResult], file: &str) {
    assert_contains(
        results,
        Finding {
            severity: Severity::Error,
            title: "rustfmt config parse error",
            message: "rustfmt config exists but could not be parsed as a TOML table",
            file: Some(file),
            inventory: false,
        },
    );
}

pub fn assert_warn_present(results: &[CheckResult], title: &str, message: &str, file: &str) {
    assert_contains(
        results,
        Finding {
            severity: Severity::Warn,
            title,
            message,
            file: Some(file),
            inventory: false,
        },
    );
}
