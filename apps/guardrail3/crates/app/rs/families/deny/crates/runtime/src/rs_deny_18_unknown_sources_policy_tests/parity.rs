use super::super::build_fixture_deny_toml;
use super::super::expected_sources_for_test;

#[test]
fn generated_sources_baseline_contains_exact_expected_unknown_source_policy() {
    let parsed = toml::from_str::<toml::Value>(&build_fixture_deny_toml("service"))
        .expect("valid deny TOML");
    let sources = parsed
        .get("sources")
        .expect("expected generated deny TOML to contain [sources]");
    let (expected_registries, expected_unknown_registry, expected_unknown_git) =
        expected_sources_for_test();

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
