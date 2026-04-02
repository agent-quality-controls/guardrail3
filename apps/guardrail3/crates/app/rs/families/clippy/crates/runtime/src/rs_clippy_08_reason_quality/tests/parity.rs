use test_support::build_fixture_clippy_toml;

#[test]
fn generated_ban_entries_use_table_format_with_reasons_across_all_sections() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("service", false, true, "", ""))
            .expect("valid clippy TOML");

    for key in [
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
    ] {
        let entries = parsed
            .get(key)
            .and_then(toml::Value::as_array)
            .cloned()
            .unwrap_or_default();
        assert!(!entries.is_empty(), "expected canonical entries in {key}");
        assert!(
            entries.iter().all(|entry| {
                entry
                    .as_table()
                    .and_then(|table| table.get("reason"))
                    .and_then(toml::Value::as_str)
                    .is_some()
            }),
            "expected canonical entries in {key} to use table format with reasons: {entries:#?}"
        );
    }
}
