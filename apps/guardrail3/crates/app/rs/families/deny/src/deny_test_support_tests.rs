use std::collections::BTreeSet;

use super::super::deny_support::{
    expected_advisory_baseline, expected_bans, expected_bans_settings, expected_graph,
    expected_licenses, expected_sources, expected_tokio_allowed_features, string_array,
};
use super::canonical_deny_toml_service;

fn deny_entry_names(parsed: &toml::Value) -> BTreeSet<String> {
    parsed
        .get("bans")
        .and_then(|value| value.get("deny"))
        .and_then(toml::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| entry.get("name").and_then(toml::Value::as_str))
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn generated_service_fixture_matches_checker_baseline_sections() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_deny_toml_service()).expect("valid deny TOML");

    let graph = parsed.get("graph").expect("graph section");
    assert_eq!(
        graph.get("all-features").and_then(toml::Value::as_bool),
        Some(expected_graph().0)
    );
    assert_eq!(
        graph
            .get("no-default-features")
            .and_then(toml::Value::as_bool),
        Some(expected_graph().1)
    );

    let bans = parsed.get("bans").expect("bans section");
    let (expected_wildcards, expected_allow_wildcard_paths, expected_highlight) =
        expected_bans_settings();
    assert_eq!(
        bans.get("wildcards").and_then(toml::Value::as_str),
        expected_wildcards.as_deref()
    );
    assert_eq!(
        bans.get("allow-wildcard-paths")
            .and_then(toml::Value::as_bool),
        Some(expected_allow_wildcard_paths)
    );
    assert_eq!(
        bans.get("highlight").and_then(toml::Value::as_str),
        expected_highlight.as_deref()
    );

    let (expected_registries, expected_unknown_registry, expected_unknown_git) = expected_sources();
    let sources = parsed.get("sources").expect("sources section");
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
    assert_eq!(
        string_array(sources.get("allow-registry")),
        expected_registries
    );

    let licenses = parsed.get("licenses").expect("licenses section");
    assert_eq!(string_array(licenses.get("allow")), expected_licenses());

    let advisories = parsed.get("advisories").expect("advisories section");
    let (expected_unmaintained, expected_yanked) = expected_advisory_baseline();
    assert_eq!(
        advisories.get("unmaintained").and_then(toml::Value::as_str),
        Some(expected_unmaintained.as_str())
    );
    assert_eq!(
        advisories.get("yanked").and_then(toml::Value::as_str),
        Some(expected_yanked.as_str())
    );

    let feature = parsed
        .get("bans")
        .and_then(|value| value.get("features"))
        .and_then(toml::Value::as_array)
        .and_then(|entries| entries.first())
        .expect("tokio feature ban");
    assert_eq!(
        string_array(feature.get("allow")),
        expected_tokio_allowed_features()
    );
}

#[test]
fn generated_service_ban_baseline_matches_expected_bans_exactly() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_deny_toml_service()).expect("valid deny TOML");
    let generated = deny_entry_names(&parsed);
    let expected = expected_bans(None).into_keys().collect::<BTreeSet<_>>();

    let only_in_expected = expected
        .difference(&generated)
        .cloned()
        .collect::<BTreeSet<_>>();
    let only_in_generated = generated
        .difference(&expected)
        .cloned()
        .collect::<BTreeSet<_>>();

    assert!(only_in_expected.is_empty());
    assert!(only_in_generated.is_empty());
}
