use super::super::super::clippy_support::parse_ban_entries;
use super::super::super::test_support::canonical_clippy_toml;

#[test]
fn generated_ban_entries_use_table_format_with_reasons_across_all_sections() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_clippy_toml()).expect("valid clippy TOML");

    for key in [
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
    ] {
        let entries = parse_ban_entries(&parsed, key);
        assert!(!entries.is_empty(), "expected canonical entries in {key}");
        assert!(
            entries
                .iter()
                .all(|entry| !entry.is_plain_string && entry.reason.as_deref().is_some()),
            "expected canonical entries in {key} to use table format with reasons: {entries:#?}"
        );
    }
}
