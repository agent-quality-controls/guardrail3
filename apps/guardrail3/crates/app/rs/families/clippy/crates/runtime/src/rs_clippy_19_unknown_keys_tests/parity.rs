use std::collections::BTreeSet;

use super::super::super::clippy_support::{
    known_top_level_keys, managed_non_threshold_keys, normalized_key_distance,
};
use super::super::super::test_support::build_fixture_clippy_toml;

#[test]
fn generated_top_level_keys_are_all_known_managed_keys() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("service", false, true, "", "")).expect("valid clippy TOML");
    let table = parsed.as_table().expect("top-level clippy table");
    let known: BTreeSet<_> = known_top_level_keys()
        .into_iter()
        .chain(managed_non_threshold_keys())
        .collect();

    for key in table.keys() {
        assert!(
            known.contains(key.as_str()),
            "unexpected canonical top-level key: {key}"
        );
        assert!(
            known
                .iter()
                .copied()
                .filter(|managed| *managed != key.as_str())
                .all(|managed| normalized_key_distance(key, managed) > 2),
            "canonical key {key} should not look like a typo of another managed key"
        );
    }
}
