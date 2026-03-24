use super::super::super::clippy_support::{is_placeholder_reason, parse_ban_entries};
use super::super::super::test_support::canonical_clippy_toml;

#[test]
fn generated_ban_entries_use_non_placeholder_reasons_across_all_sections() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_clippy_toml()).expect("valid clippy TOML");

    for key in [
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
    ] {
        let entries = parse_ban_entries(&parsed, key);
        assert!(
            entries.iter().all(|entry| {
                entry
                    .reason
                    .as_deref()
                    .is_some_and(|reason| !is_placeholder_reason(reason))
            }),
            "expected canonical reasons in {key} to stay substantive: {entries:#?}"
        );
    }
}
