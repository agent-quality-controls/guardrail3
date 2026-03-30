pub const EXTRACTOR_TYPE_BANS: &[&str] = &[
    "axum::extract::Json",
    "axum::Json",
    "axum::extract::Query",
    "axum::extract::Form",
    "axum::extract::Path",
    "axum::extract::Multipart",
    "axum::extract::ConnectInfo",
    "axum_extra::extract::CookieJar",
    "axum_extra::extract::cookie::Cookie",
    "axum_extra::extract::TypedHeader",
    "axum_extra::extract::JsonDeserializer",
    "axum_extra::extract::JsonLines",
    "axum_extra::extract::Protobuf",
    "axum_extra::extract::Cbor",
    "axum_extra::extract::MsgPack",
];

crate::define_rule_assertions!("RS-GARDE-03");

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
        "unexpected RS-GARDE-03 findings for {file}: {matching:#?}"
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
        "unexpected RS-GARDE-03 findings for {file}: {matching:#?}"
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
