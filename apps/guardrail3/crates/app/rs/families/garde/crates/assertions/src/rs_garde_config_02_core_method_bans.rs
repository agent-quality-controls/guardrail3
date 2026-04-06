pub const CORE_METHOD_BANS: &[&str] = &[
    "serde_json::from_str",
    "serde_json::from_slice",
    "serde_json::from_value",
    "serde_json::from_reader",
    "toml::from_str",
    "serde_yaml::from_str",
    "serde_yaml::from_reader",
];

crate::define_rule_assertions!("RS-GARDE-CONFIG-02");

pub fn assert_inventory(
    results: &[guardrail3_domain_report::CheckResult],
    file: &str,
    message: &str,
) {
    let findings = findings(results);
    let matching = findings
        .into_iter()
        .filter(|result| result.file() == Some(file))
        .collect::<Vec<_>>();
    assert_eq!(
        matching.len(),
        1,
        "unexpected RS-GARDE-CONFIG-02 findings for {file}: {matching:#?}"
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
        .filter(|result| result.file() == Some(file))
        .collect::<Vec<_>>();
    assert_eq!(
        matching.len(),
        1,
        "unexpected RS-GARDE-CONFIG-02 findings for {file}: {matching:#?}"
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
