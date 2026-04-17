pub(crate) fn canonical_clippy_toml() -> String {
    let method_entries = [
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
    ]
    .iter()
    .map(|path| format!("{{ path = \"{path}\" }}"))
    .collect::<Vec<_>>()
    .join(",\n    ");

    format!("disallowed-methods = [\n    {method_entries}\n]\n")
}

pub(crate) fn remove_clippy_ban_path(clippy_toml: &str, key: &str, path: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(clippy_toml)
        .expect("additional method ban fixture should parse");
    let entries = parsed
        .get_mut(key)
        .and_then(toml::Value::as_array_mut)
        .expect("additional method ban fixture should contain the requested ban array");
    entries.retain(|entry| {
        entry
            .get("path")
            .and_then(toml::Value::as_str)
            .or_else(|| entry.as_str())
            != Some(path)
    });
    toml::to_string(&parsed).expect("updated additional method ban fixture should serialize")
}
