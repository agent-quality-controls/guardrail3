use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-04";

pub fn managed_method_bans() -> Vec<&'static str> {
    guardrail3_app_rs_family_clippy::clippy_support::EXPECTED_METHOD_BANS.to_vec()
}

pub fn service_method_bans() -> Vec<&'static str> {
    guardrail3_app_rs_family_clippy::clippy_support::EXPECTED_METHOD_BANS.to_vec()
}

pub fn assert_generated_service_method_bans(actual: &[&str]) {
    let expected = service_method_bans();
    assert_eq!(actual, expected);
}

pub fn assert_golden(results: &[CheckResult], expected: &[&str], file: &str) {
    let expected_messages = expected
        .iter()
        .map(|path| format!("`{path}` is banned."))
        .collect::<Vec<_>>();
    let actual_messages = results
        .iter()
        .map(|result| result.message())
        .collect::<Vec<_>>();

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id() == ID
            && result.inventory()
            && result.severity() == Severity::Info
            && result.title() == "method ban present"
            && result.file() == Some(file)
    }));
}

pub fn assert_service_method_bans(results: &[CheckResult], file: &str) {
    let expected = service_method_bans();
    assert_golden(results, &expected, file);
}

pub fn assert_malformed_section(results: &[CheckResult], file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.title() == "disallowed-methods section malformed"
                && result.message() == "`disallowed-methods` must be an array, found table."
                && !result.inventory()
                && result.file() == Some(file)
        }),
        "expected malformed section warning: {results:#?}"
    );
    assert!(results.iter().all(|result| !result.inventory()));
}

pub fn assert_garde_disabled(results: &[CheckResult], expected: &[&str], file: &str) {
    let expected_messages = expected
        .iter()
        .map(|path| format!("`{path}` is banned."))
        .collect::<Vec<_>>();
    let actual_messages = results
        .iter()
        .map(|result| result.message())
        .collect::<Vec<_>>();

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id() == ID
            && result.inventory()
            && result.severity() == Severity::Info
            && result.title() == "method ban present"
            && result.file() == Some(file)
    }));
}

pub fn assert_missing_messages(results: &[CheckResult], expected: &[&str]) {
    let actual_errors = results
        .iter()
        .filter(|result| result.severity() == Severity::Error)
        .map(|result| result.message())
        .collect::<Vec<_>>();
    assert_eq!(actual_errors, expected);
    assert!(results.iter().all(|result| result.id() == ID));
}

pub fn expected_garde_disabled_method_bans() -> Vec<&'static str> {
    guardrail3_app_rs_family_clippy::clippy_support::EXPECTED_METHOD_BANS
        .iter()
        .filter(|path| {
            !matches!(
                **path,
                "serde_json::from_str"
                    | "serde_json::from_slice"
                    | "serde_json::from_value"
                    | "serde_json::from_reader"
                    | "reqwest::Response::json"
                    | "toml::from_str"
                    | "serde_yaml::from_str"
                    | "serde_yaml::from_reader"
                    | "serde_qs::from_str"
                    | "serde_qs::from_bytes"
                    | "serde_urlencoded::from_str"
                    | "serde_urlencoded::from_bytes"
                    | "serde_urlencoded::from_reader"
                    | "ciborium::from_reader"
                    | "ciborium::de::from_reader"
                    | "rmp_serde::from_slice"
                    | "rmp_serde::from_read"
                    | "rmp_serde::decode::from_slice"
                    | "rmp_serde::decode::from_read"
                    | "bincode::deserialize"
                    | "bincode::deserialize_from"
                    | "bincode::serde::decode_from_slice"
                    | "bincode::serde::decode_from_reader"
                    | "csv::Reader::deserialize"
                    | "csv::StringRecord::deserialize"
                    | "csv::ByteRecord::deserialize"
                    | "serde_xml_rs::from_str"
                    | "serde_xml_rs::from_reader"
                    | "quick_xml::de::from_str"
                    | "quick_xml::de::from_reader"
                    | "ron::from_str"
                    | "ron::de::from_str"
                    | "serde_cbor::from_slice"
                    | "serde_cbor::from_reader"
                    | "postcard::from_bytes"
                    | "flexbuffers::from_slice"
                    | "serde_json::Deserializer::from_str"
                    | "serde_json::Deserializer::from_slice"
                    | "serde_json::Deserializer::from_reader"
                    | "toml_edit::de::from_str"
                    | "toml_edit::de::from_slice"
                    | "toml_edit::de::from_document"
                    | "config::Config::try_deserialize"
                    | "figment::Figment::extract"
            )
        })
        .copied()
        .collect()
}

pub fn assert_plain_string_reason_quality(results: &[CheckResult], expected: &[&str], file: &str) {
    let mut actual_messages = results
        .iter()
        .filter(|result| result.id() == "RS-CLIPPY-08")
        .map(|result| result.message())
        .collect::<Vec<_>>();
    let mut expected_messages = expected.to_vec();

    actual_messages.sort();
    expected_messages.sort();
    assert_eq!(actual_messages, expected_messages);
    assert_eq!(
        results
            .iter()
            .filter(|result| result.id() == "RS-CLIPPY-08")
            .count(),
        expected_messages.len()
    );
    assert!(results.iter().all(|result| {
        result.id() == "RS-CLIPPY-08"
            && !result.inventory()
            && result.severity() == Severity::Warn
            && result.title() == "ban entry missing reason"
            && result.file() == Some(file)
    }));
}
