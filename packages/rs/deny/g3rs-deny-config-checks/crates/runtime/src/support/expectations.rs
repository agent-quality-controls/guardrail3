use std::collections::{BTreeMap, BTreeSet};

use crate::baseline as deny;

pub(crate) fn expected_advisory_baseline() -> (String, String) {
    let parsed = toml::from_str::<toml::Value>(deny::DENY_ADVISORIES.content()).ok();
    let advisories = parsed.as_ref().and_then(|value| value.get("advisories"));
    let unmaintained = advisories
        .and_then(|value| value.get("unmaintained"))
        .and_then(toml::Value::as_str)
        .unwrap_or("workspace");
    let yanked = advisories
        .and_then(|value| value.get("yanked"))
        .and_then(toml::Value::as_str)
        .unwrap_or("deny");
    (unmaintained.to_owned(), yanked.to_owned())
}

pub(crate) fn expected_graph() -> (bool, bool) {
    let parsed = toml::from_str::<toml::Value>(deny::DENY_GRAPH.content()).ok();
    let graph = parsed.as_ref().and_then(|value| value.get("graph"));
    let all_features = graph
        .and_then(|value| value.get("all-features"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(true);
    let no_default_features = graph
        .and_then(|value| value.get("no-default-features"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    (all_features, no_default_features)
}

pub(crate) fn expected_bans_settings() -> (Option<String>, bool, Option<String>) {
    let parsed = toml::from_str::<toml::Value>(deny::DENY_BANS_BASE.content()).ok();
    let bans = parsed.as_ref().and_then(|value| value.get("bans"));
    let wildcards = bans
        .and_then(|value| value.get("wildcards"))
        .and_then(toml::Value::as_str);
    let allow_wildcard_paths = bans
        .and_then(|value| value.get("allow-wildcard-paths"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(true);
    let highlight = bans
        .and_then(|value| value.get("highlight"))
        .and_then(toml::Value::as_str);
    (
        wildcards.map(str::to_owned),
        allow_wildcard_paths,
        highlight.map(str::to_owned),
    )
}

#[derive(Debug, Clone)]
pub(crate) struct BanExpectation {
    pub(crate) wrappers: BTreeSet<String>,
}

pub(crate) fn expected_bans(profile: Option<&str>) -> BTreeMap<String, BanExpectation> {
    let modules = if profile == Some("library") {
        deny::library_profile_ban_entries()
    } else {
        deny::service_profile_ban_entries()
    };

    let mut map = BTreeMap::new();
    for module in modules {
        for (name, wrappers) in parse_ban_entries(module.content()) {
            let _ = map.insert(name, BanExpectation { wrappers });
        }
    }
    map
}

pub(crate) fn expected_ban_names(profile: Option<&str>) -> BTreeSet<String> {
    expected_bans(profile).into_keys().collect()
}

pub(crate) fn expected_licenses() -> BTreeSet<String> {
    let parsed = toml::from_str::<toml::Value>(deny::DENY_LICENSES.content()).ok();
    parsed
        .as_ref()
        .and_then(|value| value.get("licenses"))
        .and_then(|value| value.get("allow"))
        .and_then(toml::Value::as_array)
        .map(|licenses| {
            licenses
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

pub(crate) const fn expected_confidence_threshold() -> f64 {
    0.8
}

pub(crate) fn expected_sources() -> (BTreeSet<String>, String, String) {
    let parsed = toml::from_str::<toml::Value>(deny::DENY_SOURCES.content()).ok();
    let sources = parsed.as_ref().and_then(|value| value.get("sources"));
    let registries = sources
        .and_then(|value| value.get("allow-registry"))
        .and_then(toml::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();
    let unknown_registry = sources
        .and_then(|value| value.get("unknown-registry"))
        .and_then(toml::Value::as_str)
        .unwrap_or("deny");
    let unknown_git = sources
        .and_then(|value| value.get("unknown-git"))
        .and_then(toml::Value::as_str)
        .unwrap_or("deny");
    (
        registries,
        unknown_registry.to_owned(),
        unknown_git.to_owned(),
    )
}

pub(crate) fn expected_tokio_allowed_features() -> BTreeSet<String> {
    let parsed = toml::from_str::<toml::Value>(deny::DENY_FEATURE_BANS_TOKIO.content()).ok();
    parsed
        .as_ref()
        .and_then(|value| value.get("bans"))
        .and_then(|value| value.get("features"))
        .and_then(toml::Value::as_array)
        .and_then(|entries| entries.first())
        .map(|entry| string_set_from_value(entry.get("allow")))
        .unwrap_or_default()
}

fn string_set_from_value(value: Option<&toml::Value>) -> BTreeSet<String> {
    value
        .and_then(toml::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn parse_ban_entries(content: &str) -> Vec<(String, BTreeSet<String>)> {
    let wrapped = format!("bans = {{ deny = [{content}] }}");
    let Ok(parsed) = toml::from_str::<toml::Value>(&wrapped) else {
        return Vec::new();
    };
    parsed
        .get("bans")
        .and_then(|value| value.get("deny"))
        .and_then(toml::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    let name = entry.get("name").and_then(toml::Value::as_str)?;
                    Some((
                        name.to_owned(),
                        string_set_from_value(entry.get("wrappers")),
                    ))
                })
                .collect()
        })
        .unwrap_or_default()
}
