use guardrail3_domain_report::CheckResult;
pub use guardrail3_domain_report::Severity;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    pub severity: Severity,
    pub title: &'a str,
    pub message: &'a str,
    pub file: Option<&'a str>,
    pub inventory: bool,
}

const ID: &str = "RS-FMT-08";

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

pub fn assert_conflict(results: &[CheckResult], file: &str) {
    let findings = findings(results);
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-FMT-08 findings: {findings:#?}"
    );
    let finding = &findings[0];
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "Conflicting rustfmt config files");
    assert_eq!(
        finding.message,
        "Both rustfmt.toml and .rustfmt.toml exist in the same directory"
    );
    assert_eq!(finding.file, Some(file));
    assert!(!finding.inventory);
}
