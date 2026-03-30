use guardrail3_domain_report::{CheckResult, Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    pub severity: Severity,
    pub title: &'a str,
    pub message: &'a str,
    pub file: Option<&'a str>,
    pub inventory: bool,
}

const ID: &str = "RS-FMT-01";

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

pub fn assert_findings(results: &[CheckResult], expected: &[Finding<'_>]) {
    assert_eq!(findings(results), expected);
}

pub fn assert_no_findings(results: &[CheckResult]) {
    assert!(findings(results).is_empty());
}

pub fn assert_missing_root_config(results: &[CheckResult]) {
    let findings = findings(results);
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-FMT-01 findings: {findings:#?}"
    );
    let finding = &findings[0];
    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "rustfmt config missing");
    assert_eq!(
        finding.message,
        "Expected rustfmt.toml or .rustfmt.toml at workspace root"
    );
    assert_eq!(finding.file, Some(""));
    assert!(!finding.inventory);
}
