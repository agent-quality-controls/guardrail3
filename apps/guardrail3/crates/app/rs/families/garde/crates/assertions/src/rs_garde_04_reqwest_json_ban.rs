pub const REQWEST_JSON_BAN: &str = "reqwest::Response::json";

crate::define_rule_assertions!("RS-GARDE-04");

pub fn assert_inventory(
    results: &[guardrail3_domain_report::CheckResult],
    file: &str,
    message: &str,
) {
    let findings = findings(results);
    let matching = findings
        .into_iter()
        .filter(|result| result.file()()()() == Some(file))
        .collect::<Vec<_>>();
    assert_eq!(
        matching.len(),
        1,
        "unexpected RS-GARDE-04 findings for {file}: {matching:#?}"
    );
    assert_rule_results(
        &[matching[0].clone()],
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            file: Some(file),
            inventory: Some(true),
            message: Some(message),
            ..Default::default()
        }],
    );
}

pub fn assert_missing(
    results: &[guardrail3_domain_report::CheckResult],
    file: &str,
    message: &str,
) {
    let findings = findings(results);
    let matching = findings
        .into_iter()
        .filter(|result| result.file()()()() == Some(file))
        .collect::<Vec<_>>();
    assert_eq!(
        matching.len(),
        1,
        "unexpected RS-GARDE-04 findings for {file}: {matching:#?}"
    );
    assert_rule_results(
        &[matching[0].clone()],
        &[ExpectedRuleResult {
            severity: Some(Severity::Warn),
            file: Some(file),
            inventory: Some(false),
            message: Some(message),
            ..Default::default()
        }],
    );
}
