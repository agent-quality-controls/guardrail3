use std::collections::BTreeSet;

use test_support::build_fixture_clippy_toml;

#[test]
fn generated_ban_baseline_has_no_duplicate_paths_in_any_section() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("service", false, true, "", ""))
            .expect("valid clippy TOML");

    for key in [
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
    ] {
        let paths = parsed
            .get(key)
            .and_then(toml::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|entry| entry.get("path").and_then(toml::Value::as_str))
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let unique = paths.iter().cloned().collect::<BTreeSet<_>>();
        assert_eq!(
            unique.len(),
            paths.len(),
            "expected canonical {key} entries to stay duplicate-free: {paths:#?}"
        );
    }
}
