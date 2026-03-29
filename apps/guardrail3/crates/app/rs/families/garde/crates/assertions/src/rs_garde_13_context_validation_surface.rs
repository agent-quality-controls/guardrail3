crate::define_rule_assertions!("RS-GARDE-13");

pub fn assert_error(
    results: &[guardrail3_domain_report::CheckResult],
    file: Option<&str>,
    title: Option<&str>,
    message: Option<&str>,
) {
    assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file,
            title,
            message,
            ..Default::default()
        }],
    );
}

pub fn assert_single_error(
    results: &[guardrail3_domain_report::CheckResult],
    file: Option<&str>,
    title: Option<&str>,
    message: Option<&str>,
) {
    let findings = findings(results);
    assert_eq!(findings.len(), 1, "unexpected RS-GARDE-13 findings: {findings:#?}");
    assert_error(results, file, title, message);
}
