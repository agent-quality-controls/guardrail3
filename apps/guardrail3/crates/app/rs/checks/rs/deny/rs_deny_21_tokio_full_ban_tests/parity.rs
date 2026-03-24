use std::collections::BTreeSet;

use super::super::super::deny_support::{
    expected_tokio_allowed_features, parse_feature_entries_in_config,
};
use super::super::super::test_support::canonical_deny_toml_service;

#[test]
fn generated_tokio_feature_policy_matches_expected_allow_and_deny_sets() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_deny_toml_service()).expect("valid deny TOML");
    let entries = parse_feature_entries_in_config(&parsed);
    let tokio = entries
        .iter()
        .find(|entry| entry.name == "tokio")
        .expect("tokio feature policy");

    assert_eq!(tokio.deny, BTreeSet::from(["full".to_owned()]));
    assert_eq!(tokio.allow, expected_tokio_allowed_features());
    assert!(tokio.unknown_keys.is_empty());
}
