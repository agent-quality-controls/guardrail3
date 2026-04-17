pub(crate) fn canonical_clippy_toml() -> String {
    let entries = [
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
    ]
    .iter()
    .map(|path| format!("{{ path = \"{path}\" }}"))
    .collect::<Vec<_>>()
    .join(",\n    ");

    format!("disallowed-types = [\n    {entries}\n]\n")
}

pub(crate) fn remove_clippy_ban_path(clippy_toml: &str, key: &str, path: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(clippy_toml)
        .expect("extractor type ban fixture should parse");
    let entries = parsed
        .get_mut(key)
        .and_then(toml::Value::as_array_mut)
        .expect("extractor type ban fixture should contain the requested ban array");
    entries.retain(|entry| {
        entry
            .get("path")
            .and_then(toml::Value::as_str)
            .or_else(|| entry.as_str())
            != Some(path)
    });
    toml::to_string(&parsed).expect("updated extractor type ban fixture should serialize")
}
