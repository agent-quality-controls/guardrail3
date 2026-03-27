use guardrail3_domain_report::{CheckResult, Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    pub severity: Severity,
    pub title: &'a str,
    pub message: &'a str,
    pub file: Option<&'a str>,
    pub inventory: bool,
}

const ID: &str = "RS-FMT-07";

#[must_use]
pub fn findings(results: &[CheckResult]) -> Vec<Finding<'_>> {
    results
        .iter()
        .filter(|result| result.id == ID)
        .map(|result| Finding {
            severity: result.severity,
            title: result.title.as_str(),
            message: result.message.as_str(),
            file: result.file.as_deref(),
            inventory: result.inventory,
        })
        .collect()
}

pub fn assert_findings(results: &[CheckResult], expected: &[Finding<'_>]) {
    assert_eq!(findings(results), expected);
}

pub fn assert_no_findings(results: &[CheckResult]) {
    assert!(findings(results).is_empty());
}

pub fn assert_ignore_escape_hatch(results: &[CheckResult], message: &str, file: &str) {
    let findings = findings(results);
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-FMT-07 findings: {findings:#?}"
    );
    let finding = &findings[0];
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "rustfmt ignore escape hatch");
    assert_eq!(finding.message, message);
    assert_eq!(finding.file, Some(file));
    assert!(!finding.inventory);
}
