use std::collections::BTreeSet;

use super::super::clippy_support::{
    EXPECTED_MACRO_BANS, THRESHOLD_EXPECTATIONS, expected_method_bans, expected_type_bans,
    parse_ban_entries, threshold_value,
};
use super::canonical_clippy_toml;

fn paths_for_key(parsed: &toml::Value, key: &str) -> BTreeSet<String> {
    parse_ban_entries(parsed, key)
        .into_iter()
        .map(|entry| entry.path)
        .collect()
}

#[test]
fn generated_service_fixture_matches_checker_expectations() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_clippy_toml()).expect("valid clippy TOML");

    for expectation in THRESHOLD_EXPECTATIONS {
        assert_eq!(
            threshold_value(&parsed, expectation.key),
            Some(expectation.expected),
            "threshold drift for {}",
            expectation.key
        );
    }

    assert_eq!(
        parsed
            .get("avoid-breaking-exported-api")
            .and_then(toml::Value::as_bool),
        Some(false)
    );
    assert_eq!(
        parsed
            .get("allow-dbg-in-tests")
            .and_then(toml::Value::as_bool),
        Some(false)
    );
    assert_eq!(
        parsed
            .get("allow-print-in-tests")
            .and_then(toml::Value::as_bool),
        Some(false)
    );

    let expected_methods = expected_method_bans(true)
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        paths_for_key(&parsed, "disallowed-methods"),
        expected_methods
    );

    let expected_types = expected_type_bans(None, true)
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    assert_eq!(paths_for_key(&parsed, "disallowed-types"), expected_types);

    let expected_macros = EXPECTED_MACRO_BANS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(paths_for_key(&parsed, "disallowed-macros"), expected_macros);
}
