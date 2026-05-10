pub(crate) fn canonical_clippy_toml() -> String {
    let entries = [
        "serde_json::from_str",
        "serde_json::from_slice",
        "serde_json::from_value",
        "serde_json::from_reader",
        "toml::from_str",
        "serde_yaml::from_str",
        "serde_yaml::from_reader",
    ]
    .iter()
    .map(|path| format!("{{ path = \"{path}\" }}"))
    .collect::<Vec<_>>()
    .join(",\n    ");

    format!("disallowed-methods = [\n    {entries}\n]\n")
}

#[expect(
    clippy::disallowed_methods,
    reason = "test fixture mutator: synthesizes clippy.toml variants for unit tests; not a runtime parser"
)]
pub(crate) fn remove_clippy_ban_path(clippy_toml: &str, key: &str, path: &str) -> String {
    let mut parsed =
        toml::from_str::<toml::Value>(clippy_toml).expect("core method ban fixture should parse");
    let entries = parsed
        .get_mut(key)
        .and_then(toml::Value::as_array_mut)
        .expect("core method ban fixture should contain the requested ban array");
    entries.retain(|entry| {
        entry
            .get("path")
            .and_then(toml::Value::as_str)
            .or_else(|| entry.as_str())
            != Some(path)
    });
    toml::to_string(&parsed).expect("updated core method ban fixture should serialize")
}
