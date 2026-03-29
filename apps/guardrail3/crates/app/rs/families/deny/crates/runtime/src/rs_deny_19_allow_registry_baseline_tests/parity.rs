use super::super::build_fixture_deny_toml;
use super::super::expected_sources_for_test;

#[test]
fn generated_sources_baseline_contains_exact_expected_registry_allow_list() {
    let parsed = toml::from_str::<toml::Value>(&build_fixture_deny_toml("service"))
        .expect("valid deny TOML");
    let sources = parsed
        .get("sources")
        .expect("expected generated deny TOML to contain [sources]");
    let (expected_registries, _, _) = expected_sources_for_test();

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
}
