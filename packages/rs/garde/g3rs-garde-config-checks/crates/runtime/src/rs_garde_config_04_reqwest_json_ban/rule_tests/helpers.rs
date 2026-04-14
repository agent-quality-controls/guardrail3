pub(crate) fn canonical_clippy_toml() -> String {
    crate::test_support::canonical_clippy_toml()
}

pub(crate) fn remove_clippy_ban_path(clippy_toml: &str, key: &str, path: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(clippy_toml).expect("valid clippy TOML");
    let entries = parsed
        .get_mut(key)
        .and_then(toml::Value::as_array_mut)
        .expect("expected ban array");
    entries.retain(|entry| {
        entry
            .get("path")
            .and_then(toml::Value::as_str)
            .or_else(|| entry.as_str())
            != Some(path)
    });
    toml::to_string(&parsed).expect("serialize clippy TOML")
}
