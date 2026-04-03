use std::collections::BTreeSet;

use super::helpers::build_fixture_deny_toml;
use super::helpers::expected_ban_names_for_test;

fn deny_entry_names(parsed: &toml::Value) -> BTreeSet<String> {
    parsed
        .get("bans")
        .and_then(|value| value.get("deny"))
        .and_then(toml::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    entry
                        .get("name")
                        .or_else(|| entry.get("crate"))
                        .and_then(toml::Value::as_str)
                })
                .map(|name| name.split('@').next().unwrap_or(name).to_owned())
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn generated_service_ban_set_matches_rule_baseline_exactly() {
    let parsed = toml::from_str::<toml::Value>(&build_fixture_deny_toml("service"))
        .expect("valid deny TOML");
    let generated = deny_entry_names(&parsed);
    let expected = expected_ban_names_for_test(Some("service"));

    assert!(
        expected.difference(&generated).next().is_none(),
        "missing generated service bans: {:?}",
        expected.difference(&generated).collect::<Vec<_>>()
    );
    assert!(
        generated.difference(&expected).next().is_none(),
        "unexpected generated service bans: {:?}",
        generated.difference(&expected).collect::<Vec<_>>()
    );
}

#[test]
fn generated_library_ban_set_matches_rule_baseline_exactly() {
    let parsed = toml::from_str::<toml::Value>(&build_fixture_deny_toml("library"))
        .expect("valid deny TOML");
    let generated = deny_entry_names(&parsed);
    let expected = expected_ban_names_for_test(Some("library"));

    assert!(
        expected.difference(&generated).next().is_none(),
        "missing generated library bans: {:?}",
        expected.difference(&generated).collect::<Vec<_>>()
    );
    assert!(
        generated.difference(&expected).next().is_none(),
        "unexpected generated library bans: {:?}",
        generated.difference(&expected).collect::<Vec<_>>()
    );
}
