use std::collections::BTreeSet;

use super::super::super::deny_support::expected_bans;
use super::super::super::test_support::{canonical_deny_toml_library, canonical_deny_toml_service};

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
    let parsed =
        toml::from_str::<toml::Value>(&canonical_deny_toml_service()).expect("valid deny TOML");
    let generated = deny_entry_names(&parsed);
    let expected = expected_bans(Some("service"))
        .into_keys()
        .collect::<BTreeSet<_>>();

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
    let parsed =
        toml::from_str::<toml::Value>(&canonical_deny_toml_library()).expect("valid deny TOML");
    let generated = deny_entry_names(&parsed);
    let expected = expected_bans(Some("library"))
        .into_keys()
        .collect::<BTreeSet<_>>();

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
