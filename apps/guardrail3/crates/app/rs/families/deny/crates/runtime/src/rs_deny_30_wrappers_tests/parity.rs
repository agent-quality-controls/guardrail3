use std::collections::BTreeMap;

use super::super::expected_ban_wrappers_for_test;
use super::super::build_fixture_deny_toml;

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
        toml::from_str::<toml::Value>(&build_fixture_deny_toml("service")).expect("valid deny TOML");
    let generated = generated_wrapper_map(&parsed);
    let expected = expected_ban_wrappers_for_test(Some("service"));

    assert_eq!(generated, expected);
}
