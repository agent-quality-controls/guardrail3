use super::super::super::deny_support::parse_feature_entries_in_config;
use super::super::super::test_support::canonical_deny_toml_service;

#[test]
fn generated_feature_ban_baseline_contains_only_the_canonical_tokio_entry() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_deny_toml_service()).expect("valid deny TOML");
    let entries = parse_feature_entries_in_config(&parsed);

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "tokio");
}
