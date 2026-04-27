pub(crate) fn canonical_clippy_toml() -> String {
    "disallowed-methods = [\n    { path = \"reqwest::Response::json\" }\n]\n".to_owned()
}

pub(crate) fn remove_clippy_ban_path(clippy_toml: &str, key: &str, path: &str) -> String {
    let mut parsed =
        toml::from_str::<toml::Value>(clippy_toml).expect("reqwest json ban fixture should parse");
    let entries = parsed
        .get_mut(key)
        .and_then(toml::Value::as_array_mut)
        .expect("reqwest json ban fixture should contain the requested ban array");
    entries.retain(|entry| {
        entry
            .get("path")
            .and_then(toml::Value::as_str)
            .or_else(|| entry.as_str())
            != Some(path)
    });
    toml::to_string(&parsed).expect("updated reqwest json ban fixture should serialize")
}
