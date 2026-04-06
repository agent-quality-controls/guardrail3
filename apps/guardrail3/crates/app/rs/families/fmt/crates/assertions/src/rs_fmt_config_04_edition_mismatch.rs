use guardrail3_domain_report::{CheckResult, Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    pub severity: Severity,
    pub title: &'a str,
    pub message: &'a str,
    pub file: Option<&'a str>,
    pub inventory: bool,
}

const ID: &str = "RS-FMT-CONFIG-04";

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

pub fn assert_mismatch(results: &[CheckResult], rustfmt_edition: &str, cargo_edition: &str) {
    let findings = findings(results);
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-FMT-CONFIG-04 findings: {findings:#?}"
    );
    let finding = &findings[0];
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "rustfmt edition differs from Cargo edition");
    assert_eq!(
        finding.message,
        format!(
            "rustfmt edition `{rustfmt_edition}` differs from Cargo edition `{cargo_edition}`. Update `edition` in rustfmt.toml to `{cargo_edition}`."
        )
    );
    assert_eq!(finding.file, Some("rustfmt.toml"));
    assert!(!finding.inventory);
}

pub fn assert_error(results: &[CheckResult], title: &str, message: &str) {
    let findings = findings(results);
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-FMT-CONFIG-04 findings: {findings:#?}"
    );
    let finding = &findings[0];
    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, title);
    assert_eq!(finding.message, message);
    assert_eq!(finding.file, Some("Cargo.toml"));
    assert!(!finding.inventory);
}

pub fn assert_malformed_root_manifest_error(results: &[CheckResult]) {
    assert_error(
        results,
        "Cargo.toml parse error",
        "rustfmt edition checks require a parseable root Cargo.toml.",
    );
}

pub fn assert_missing_root_edition_error(results: &[CheckResult]) {
    assert_error(
        results,
        "Cargo.toml edition missing",
        "rustfmt edition checks require `[workspace.package].edition` or `[package].edition` in root Cargo.toml.",
    );
}

pub fn assert_missing_root_manifest_error(results: &[CheckResult]) {
    assert_error(
        results,
        "Cargo.toml missing",
        "rustfmt edition checks require a root Cargo.toml with workspace or package edition.",
    );
}
