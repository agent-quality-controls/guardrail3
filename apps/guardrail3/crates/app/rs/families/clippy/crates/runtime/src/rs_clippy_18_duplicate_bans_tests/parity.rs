use std::collections::BTreeSet;

use super::super::super::clippy_support::parse_ban_entries;
use super::super::super::test_support::canonical_clippy_toml;

#[test]
fn generated_ban_baseline_has_no_duplicate_paths_in_any_section() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_clippy_toml()).expect("valid clippy TOML");

    for key in [
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
    ] {
        let paths = parse_ban_entries(&parsed, key)
            .into_iter()
            .map(|entry| entry.path)
            .collect::<Vec<_>>();
        let unique = paths.iter().cloned().collect::<BTreeSet<_>>();
        assert_eq!(
            unique.len(),
            paths.len(),
            "expected canonical {key} entries to stay duplicate-free: {paths:#?}"
        );
    }
}
