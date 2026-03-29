crate::define_rule_assertions!("RS-GARDE-07");

pub fn assert_error(
    results: &[guardrail3_domain_report::CheckResult],
    file: Option<&str>,
    line: Option<usize>,
    title: Option<&str>,
) {
    assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file,
            line,
            title,
            ..Default::default()
        }],
    );
}

pub fn assert_single_error(
    results: &[guardrail3_domain_report::CheckResult],
    file: Option<&str>,
    line: Option<usize>,
    title: Option<&str>,
) {
    let findings = findings(results);
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-GARDE-07 findings: {findings:#?}"
    );
    assert_error(results, file, line, title);
}
