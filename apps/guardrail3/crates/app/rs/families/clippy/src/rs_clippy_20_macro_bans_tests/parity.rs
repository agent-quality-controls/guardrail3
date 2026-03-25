use std::collections::BTreeSet;

use super::super::super::clippy_support::{EXPECTED_MACRO_BANS, parse_ban_entries};
use super::super::super::test_support::canonical_clippy_toml;

#[test]
fn generated_macro_ban_set_matches_rule_baseline() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_clippy_toml()).expect("valid clippy TOML");
    let actual = parse_ban_entries(&parsed, "disallowed-macros")
        .into_iter()
        .map(|entry| entry.path)
        .collect::<BTreeSet<_>>();
    let expected = EXPECTED_MACRO_BANS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}
