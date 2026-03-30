crate::define_result_assertions!("RS-DENY-01");

pub fn assert_parse_error(results: &[guardrail3_domain_report::CheckResult], file: &str) {
    let parse_errors = findings(results)
        .into_iter()
        .filter(|finding| finding.title == "deny config parse error")
        .collect::<Vec<_>>();
    assert_eq!(
        parse_errors.len(),
        1,
        "unexpected RS-DENY-01 parse errors: {parse_errors:#?}"
    );
    let finding = &parse_errors[0];
    assert_eq!(finding.severity, guardrail3_domain_report::Severity::Error);
    assert_eq!(finding.file, Some(file));
    assert!(!finding.inventory);
    assert!(
        finding
            .message
            .starts_with(&format!("`{file}` could not be parsed: "))
    );
}

pub fn assert_policy_context_parse_error(
    results: &[guardrail3_domain_report::CheckResult],
    expected_fragment: &str,
) {
    let parse_errors = findings(results)
        .into_iter()
        .filter(|finding| finding.title == "deny policy context is not parseable")
        .collect::<Vec<_>>();
    assert_eq!(
        parse_errors.len(),
        1,
        "unexpected RS-DENY-01 policy-context parse errors: {parse_errors:#?}"
    );
    let finding = &parse_errors[0];
    assert_eq!(finding.severity, guardrail3_domain_report::Severity::Error);
    assert_eq!(finding.file, Some("guardrail3.toml"));
    assert!(!finding.inventory);
    assert!(
        finding.message.contains(expected_fragment),
        "expected message to contain `{expected_fragment}`, got {:?}",
        finding.message
    );
}

pub fn assert_no_findings_for(results: &[guardrail3_domain_report::CheckResult], rule_id: &str) {
    let findings = results
        .iter()
        .filter(|result| result.id() == rule_id)
        .collect::<Vec<_>>();
    assert!(
        findings.is_empty(),
        "expected no {rule_id} findings, got {findings:#?}"
    );
}
