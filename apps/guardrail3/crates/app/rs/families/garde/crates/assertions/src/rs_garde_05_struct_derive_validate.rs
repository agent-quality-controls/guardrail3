crate::define_rule_assertions!("RS-GARDE-05");

pub fn assert_single_error(
    results: &[guardrail3_domain_report::CheckResult],
    file: Option<&str>,
    line: Option<usize>,
    title: Option<&str>,
    message: Option<&str>,
) {
    let findings = findings(results);
    assert_eq!(findings.len(), 1, "unexpected RS-GARDE-05 findings: {findings:#?}");
    assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file,
            line,
            title,
            message,
            ..Default::default()
        }],
    );
}
