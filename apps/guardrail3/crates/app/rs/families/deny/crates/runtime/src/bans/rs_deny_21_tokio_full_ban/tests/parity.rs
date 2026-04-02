use std::collections::BTreeSet;

use super::super::build_fixture_deny_toml;
use super::super::{expected_tokio_allowed_features_for_test, parse_feature_entries_for_test};

#[test]
fn generated_tokio_feature_policy_matches_expected_allow_and_deny_sets() {
    let parsed = toml::from_str::<toml::Value>(&build_fixture_deny_toml("service"))
        .expect("valid deny TOML");
    let entries = parse_feature_entries_for_test(&parsed);
    let tokio = entries
        .iter()
        .find(|entry| entry.name == "tokio")
        .expect("tokio feature policy");

    assert_eq!(tokio.deny, BTreeSet::from(["full".to_owned()]));
    assert_eq!(tokio.allow, expected_tokio_allowed_features_for_test());
    assert!(tokio.unknown_keys.is_empty());
}
