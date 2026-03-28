use test_support::build_fixture_clippy_toml;

#[test]
fn generated_ban_entries_use_non_placeholder_reasons_across_all_sections() {
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
        assert!(
            entries.iter().all(|entry| {
                entry
                    .as_table()
                    .and_then(|table| table.get("reason"))
                    .and_then(toml::Value::as_str)
                    .is_some_and(|reason| !is_placeholder_reason(reason))
            }),
            "expected canonical reasons in {key} to stay substantive: {entries:#?}"
        );
    }
}

fn is_placeholder_reason(reason: &str) -> bool {
    let normalized = reason.trim().to_ascii_lowercase();
    normalized.is_empty()
        || normalized.len() < 10
        || matches!(
            normalized.as_str(),
            "todo" | "fixme" | "fix later" | "tbd" | "..." | "reason"
        )
}
