pub const ADDITIONAL_METHOD_BANS: &[&str] = &[
    "serde_qs::from_str",
    "serde_qs::from_bytes",
    "serde_urlencoded::from_str",
    "serde_urlencoded::from_bytes",
    "serde_urlencoded::from_reader",
    "ciborium::from_reader",
    "ciborium::de::from_reader",
    "rmp_serde::from_slice",
    "rmp_serde::from_read",
    "rmp_serde::decode::from_slice",
    "rmp_serde::decode::from_read",
    "bincode::deserialize",
    "bincode::deserialize_from",
    "bincode::serde::decode_from_slice",
    "bincode::serde::decode_from_reader",
    "csv::Reader::deserialize",
    "csv::StringRecord::deserialize",
    "csv::ByteRecord::deserialize",
    "serde_xml_rs::from_str",
    "serde_xml_rs::from_reader",
    "quick_xml::de::from_str",
    "quick_xml::de::from_reader",
    "ron::from_str",
    "ron::de::from_str",
    "serde_cbor::from_slice",
    "serde_cbor::from_reader",
    "postcard::from_bytes",
    "flexbuffers::from_slice",
    "serde_json::Deserializer::from_str",
    "serde_json::Deserializer::from_slice",
    "serde_json::Deserializer::from_reader",
    "toml_edit::de::from_str",
    "toml_edit::de::from_slice",
    "toml_edit::de::from_document",
    "config::Config::try_deserialize",
    "figment::Figment::extract",
];

crate::define_rule_assertions!("RS-GARDE-06");

pub fn assert_inventory(
    results: &[guardrail3_domain_report::CheckResult],
    file: &str,
    message: &str,
) {
    let findings = findings(results);
    let matching = findings
        .into_iter()
        .filter(|result| result.file.as_deref() == Some(file))
        .collect::<Vec<_>>();
    assert_eq!(
        matching.len(),
        1,
        "unexpected RS-GARDE-06 findings for {file}: {matching:#?}"
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
        .filter(|result| result.file.as_deref() == Some(file))
        .collect::<Vec<_>>();
    assert_eq!(
        matching.len(),
        1,
        "unexpected RS-GARDE-06 findings for {file}: {matching:#?}"
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
