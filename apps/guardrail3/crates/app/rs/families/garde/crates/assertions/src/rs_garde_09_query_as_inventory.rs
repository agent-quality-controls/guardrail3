crate::define_rule_assertions!("RS-GARDE-09");

pub fn assert_documented_hit(
    results: &[guardrail3_domain_report::CheckResult],
    file: &str,
    line: usize,
    message_contains: &str,
) {
    let findings = findings(results);
    assert!(
        findings.iter().any(|result| {
            result.severity() == Severity::Warn
                && result.file() == Some(file)
                && result.line() == Some(line)
                && !result.inventory()
                && result.message().contains(message_contains)
        }),
        "missing documented RS-GARDE-09 hit for {file}:{line}: {findings:#?}"
    );
}

pub fn assert_count_summary(results: &[guardrail3_domain_report::CheckResult], message: &str) {
    let findings = findings(results);
    assert!(
        findings.iter().any(|result| {
            result.severity() == Severity::Warn
                && result.title() == "sqlx query_as count"
                && result.file().is_none()
                && !result.inventory()
                && result.message() == message
        }),
        "missing RS-GARDE-09 count summary: {findings:#?}"
    );
}

pub fn assert_inventory_hit(
    results: &[guardrail3_domain_report::CheckResult],
    file: &str,
    line: usize,
    message: &str,
) {
    let findings = findings(results);
    let matching = findings
        .into_iter()
        .filter(|result| result.file() == Some(file) && result.line() == Some(line))
        .collect::<Vec<_>>();
    assert_eq!(
        matching.len(),
        1,
        "unexpected RS-GARDE-09 findings for {file}:{line}: {matching:#?}"
    );
    assert_rule_results(
        &[matching[0].clone()],
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            file: Some(file),
            line: Some(line),
            inventory: Some(true),
            message: Some(message),
            ..Default::default()
        }],
    );
}

pub fn assert_single_inventory_hit(
    results: &[guardrail3_domain_report::CheckResult],
    file: &str,
    line: usize,
    message: &str,
) {
    let findings = findings(results);
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-GARDE-09 findings: {findings:#?}"
    );
    assert_inventory_hit(results, file, line, message);
}
