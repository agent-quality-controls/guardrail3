crate::define_rule_assertions!("RS-GARDE-11");

pub fn assert_error(
    results: &[guardrail3_domain_report::CheckResult],
    file: Option<&str>,
    line: Option<usize>,
    title: Option<&str>,
    message: Option<&str>,
) {
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

pub fn assert_single_error(
    results: &[guardrail3_domain_report::CheckResult],
    file: Option<&str>,
    line: Option<usize>,
    title: Option<&str>,
    message: Option<&str>,
) {
    let findings = findings(results);
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-GARDE-11 findings: {findings:#?}"
    );
    assert_error(results, file, line, title, message);
}

pub fn assert_nested_dive_quiet(results: &[guardrail3_domain_report::CheckResult]) {
    crate::common::assert_rule_quiet(results, "RS-GARDE-12");
}
