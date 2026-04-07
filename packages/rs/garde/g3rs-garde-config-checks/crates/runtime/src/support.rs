use std::collections::BTreeSet;

use cargo_toml_parser::CargoToml;
use clippy_toml_parser::{ClippyToml, DisallowedPath};
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) const CORE_METHOD_BANS: &[&str] = &[
    "serde_json::from_str",
    "serde_json::from_slice",
    "serde_json::from_value",
    "serde_json::from_reader",
    "toml::from_str",
    "serde_yaml::from_str",
    "serde_yaml::from_reader",
];

pub(crate) const REQWEST_JSON_BAN: &str = "reqwest::Response::json";

pub(crate) const ADDITIONAL_METHOD_BANS: &[&str] = &[
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

pub(crate) const EXTRACTOR_TYPE_BANS: &[&str] = &[
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

pub(crate) fn has_garde_dependency(cargo: &CargoToml) -> bool {
    cargo.dependencies.contains_key("garde")
        || cargo
            .workspace
            .as_ref()
            .is_some_and(|workspace| workspace.dependencies.contains_key("garde"))
}

pub(crate) fn disallowed_method_paths(clippy: &ClippyToml) -> BTreeSet<String> {
    extract_disallowed_paths(&clippy.disallowed_methods)
}

pub(crate) fn disallowed_type_paths(clippy: &ClippyToml) -> BTreeSet<String> {
    extract_disallowed_paths(&clippy.disallowed_types)
}

pub(crate) fn missing_bans<'a>(found: &BTreeSet<String>, expected: &'a [&'a str]) -> Vec<&'a str> {
    expected
        .iter()
        .filter(|path| !found.contains(**path))
        .copied()
        .collect()
}

pub(crate) fn error(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
}

pub(crate) fn warn(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Warn,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
}

pub(crate) fn info(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
    .into_inventory()
}

fn extract_disallowed_paths(entries: &[DisallowedPath]) -> BTreeSet<String> {
    entries
        .iter()
        .map(|entry| match entry {
            DisallowedPath::Simple(path) => path.clone(),
            DisallowedPath::Detailed(detail) => detail.path.clone(),
        })
        .collect()
}
