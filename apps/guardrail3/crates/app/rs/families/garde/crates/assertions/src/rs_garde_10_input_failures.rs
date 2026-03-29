crate::define_rule_assertions!("RS-GARDE-10");

pub fn assert_error_contains(
    results: &[guardrail3_domain_report::CheckResult],
    file: &str,
    message_contains: &str,
) {
    assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(file),
            message_contains: Some(message_contains),
            ..Default::default()
        }],
    );
}

pub fn assert_error_file(results: &[guardrail3_domain_report::CheckResult], file: &str) {
    assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(file),
            ..Default::default()
        }],
    );
}

pub fn assert_single_error_contains(
    results: &[guardrail3_domain_report::CheckResult],
    file: &str,
    message_contains: &str,
) {
    let findings = findings(results);
    assert_eq!(findings.len(), 1, "unexpected RS-GARDE-10 findings: {findings:#?}");
    assert_error_contains(results, file, message_contains);
}

pub fn assert_single_error_file(results: &[guardrail3_domain_report::CheckResult], file: &str) {
    let findings = findings(results);
    assert_eq!(findings.len(), 1, "unexpected RS-GARDE-10 findings: {findings:#?}");
    assert_error_file(results, file);
}
