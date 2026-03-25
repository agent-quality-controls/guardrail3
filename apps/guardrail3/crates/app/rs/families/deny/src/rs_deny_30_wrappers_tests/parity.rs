use std::collections::BTreeMap;

use super::super::super::deny_support::expected_bans;
use super::super::super::test_support::canonical_deny_toml_service;

fn generated_wrapper_map(
    parsed: &toml::Value,
) -> BTreeMap<String, std::collections::BTreeSet<String>> {
    parsed
        .get("bans")
        .and_then(|value| value.get("deny"))
        .and_then(toml::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    let name = entry
                        .get("name")
                        .or_else(|| entry.get("crate"))
                        .and_then(toml::Value::as_str)?;
                    let wrappers = entry
                        .get("wrappers")
                        .and_then(toml::Value::as_array)
                        .map(|items| {
                            items
                                .iter()
                                .filter_map(toml::Value::as_str)
                                .map(str::to_owned)
                                .collect()
                        })
                        .unwrap_or_default();
                    Some((name.split('@').next().unwrap_or(name).to_owned(), wrappers))
                })
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn generated_ban_wrappers_match_expected_canonical_wrapper_policy() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_deny_toml_service()).expect("valid deny TOML");
    let generated = generated_wrapper_map(&parsed);
    let expected = expected_bans(Some("service"))
        .into_iter()
        .map(|(name, expectation)| (name, expectation.wrappers))
        .collect::<BTreeMap<_, _>>();

    assert_eq!(generated, expected);
}
