use guardrail3_reason_policy::reason_text_is_useful;
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
                    .is_some_and(reason_text_is_useful)
            }),
            "expected canonical reasons in {key} to stay substantive: {entries:#?}"
        );
    }
}
