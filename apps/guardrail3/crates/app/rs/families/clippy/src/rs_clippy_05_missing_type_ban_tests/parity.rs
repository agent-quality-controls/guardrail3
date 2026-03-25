use std::collections::BTreeSet;

use super::super::super::clippy_support::{expected_type_bans, parse_ban_entries};
use super::super::super::test_support::canonical_clippy_toml;
use guardrail3_domain_modules::clippy::build_clippy_toml;

#[test]
fn generated_service_type_ban_set_matches_rule_baseline() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_clippy_toml()).expect("valid clippy TOML");
    let actual = parse_ban_entries(&parsed, "disallowed-types")
        .into_iter()
        .map(|entry| entry.path)
        .collect::<BTreeSet<_>>();
    let expected = expected_type_bans(None, true)
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}

#[test]
fn generated_library_type_ban_set_matches_rule_baseline() {
    let parsed = toml::from_str::<toml::Value>(&build_clippy_toml("library", false, true, "", ""))
        .expect("valid clippy TOML");
    let actual = parse_ban_entries(&parsed, "disallowed-types")
        .into_iter()
        .map(|entry| entry.path)
        .collect::<BTreeSet<_>>();
    let expected = expected_type_bans(Some("library"), true)
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}
