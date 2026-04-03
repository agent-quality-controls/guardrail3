use super::helpers::build_fixture_deny_toml;
use super::helpers::parse_feature_entries_for_test;

#[test]
fn generated_feature_ban_baseline_contains_only_the_canonical_tokio_entry() {
    let parsed = toml::from_str::<toml::Value>(&build_fixture_deny_toml("service"))
        .expect("valid deny TOML");
    let entries = parse_feature_entries_for_test(&parsed);

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "tokio");
}
