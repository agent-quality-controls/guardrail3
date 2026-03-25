use super::super::super::deny_support::expected_sources;
use super::super::super::test_support::canonical_deny_toml_service;

#[test]
fn generated_sources_baseline_contains_exact_expected_unknown_source_policy() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_deny_toml_service()).expect("valid deny TOML");
    let sources = parsed.get("sources").expect("sources section");
    let (expected_registries, expected_unknown_registry, expected_unknown_git) = expected_sources();

    let actual_registries = sources
        .get("allow-registry")
        .and_then(toml::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect::<std::collections::BTreeSet<_>>()
        })
        .unwrap_or_default();

    assert_eq!(actual_registries, expected_registries);
    assert_eq!(
        sources
            .get("unknown-registry")
            .and_then(toml::Value::as_str),
        Some(expected_unknown_registry.as_str())
    );
    assert_eq!(
        sources.get("unknown-git").and_then(toml::Value::as_str),
        Some(expected_unknown_git.as_str())
    );
}
