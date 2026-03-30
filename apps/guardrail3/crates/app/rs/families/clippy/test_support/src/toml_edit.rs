pub fn remove_ban_path(clippy_toml: &str, key: &str, path: &str) -> String {
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

pub fn prepend_ban_path(clippy_toml: &str, key: &str, path: &str, reason: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(clippy_toml).expect("valid clippy TOML");
    let entries = parsed
        .get_mut(key)
        .and_then(toml::Value::as_array_mut)
        .expect("expected ban array");
    let mut entry = toml::map::Map::new();
    let _ = entry.insert("path".to_owned(), toml::Value::String(path.to_owned()));
    let _ = entry.insert("reason".to_owned(), toml::Value::String(reason.to_owned()));
    entries.insert(0, toml::Value::Table(entry));
    toml::to_string(&parsed).expect("serialize clippy TOML")
}

pub fn replace_ban_entry_with_string(clippy_toml: &str, key: &str, path: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(clippy_toml).expect("valid clippy TOML");
    let entries = parsed
        .get_mut(key)
        .and_then(toml::Value::as_array_mut)
        .expect("expected ban array");

    let replacement_index = entries
        .iter()
        .position(|entry| {
            entry
                .get("path")
                .and_then(toml::Value::as_str)
                .or_else(|| entry.as_str())
                == Some(path)
        })
        .expect("expected path in ban array");
    entries[replacement_index] = toml::Value::String(path.to_owned());

    toml::to_string(&parsed).expect("serialize clippy TOML")
}
