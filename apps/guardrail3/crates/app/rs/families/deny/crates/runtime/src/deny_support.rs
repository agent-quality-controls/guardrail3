use std::collections::{BTreeMap, BTreeSet};

use guardrail3_domain_modules::deny::{
    self, DENY_ADVISORIES, DENY_BANS_BASE, DENY_LICENSES, DENY_SOURCES,
};

#[derive(Debug, Clone)]
pub struct BanExpectation {
    pub(crate) wrappers: BTreeSet<String>,
}

pub fn parsed_table(
    config: &super::facts::DenyConfigFacts,
) -> Option<&toml::map::Map<String, toml::Value>> {
    config.parsed.as_ref()?.as_table()
}

pub fn section<'a>(
    config: &'a super::facts::DenyConfigFacts,
    name: &str,
) -> Option<&'a toml::Value> {
    config.parsed.as_ref()?.get(name)
}

pub fn ban_name(entry: &toml::Value) -> Option<String> {
    if let Some(name) = entry.get("name").and_then(toml::Value::as_str) {
        return Some(name.to_owned());
    }
    if let Some(crate_name) = entry.get("crate").and_then(toml::Value::as_str) {
        return Some(
            crate_name
                .split('@')
                .next()
                .unwrap_or(crate_name)
                .to_owned(),
        );
    }
    entry.as_str().map(str::to_owned)
}

pub fn join_set(values: &BTreeSet<String>) -> String {
    values.iter().cloned().collect::<Vec<_>>().join(", ")
}

pub fn expected_bans(profile: Option<&str>) -> BTreeMap<String, BanExpectation> {
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

pub fn expected_ban_names(profile: Option<&str>) -> BTreeSet<String> {
    expected_bans(profile).into_keys().collect()
}

pub fn expected_licenses() -> BTreeSet<String> {
    let parsed = toml::from_str::<toml::Value>(DENY_LICENSES.content()).ok();
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

pub fn expected_confidence_threshold() -> f64 {
    0.8
}

pub fn expected_advisory_baseline() -> (String, String) {
    let parsed = toml::from_str::<toml::Value>(DENY_ADVISORIES.content()).ok();
    let advisories = parsed.as_ref().and_then(|value| value.get("advisories"));
    let unmaintained = advisories
        .and_then(|value| value.get("unmaintained"))
        .and_then(toml::Value::as_str)
        .unwrap_or("workspace");
    let yanked = advisories
        .and_then(|value| value.get("yanked"))
        .and_then(toml::Value::as_str)
        .unwrap_or("warn");
    (unmaintained.to_owned(), yanked.to_owned())
}

pub fn expected_sources() -> (BTreeSet<String>, String, String) {
    let parsed = toml::from_str::<toml::Value>(DENY_SOURCES.content()).ok();
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

pub fn expected_graph() -> (bool, bool) {
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

pub fn expected_bans_settings() -> (Option<String>, bool, Option<String>) {
    let parsed = toml::from_str::<toml::Value>(DENY_BANS_BASE.content()).ok();
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

pub struct FeatureConfigEntry {
    pub(crate) name: String,
    pub(crate) deny: BTreeSet<String>,
    pub(crate) allow: BTreeSet<String>,
    pub(crate) unknown_keys: Vec<String>,
}

pub fn string_array(value: Option<&toml::Value>) -> BTreeSet<String> {
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

pub fn parse_feature_entries_in_config(parsed: &toml::Value) -> Vec<FeatureConfigEntry> {
    parsed
        .get("bans")
        .and_then(|value| value.get("features"))
        .and_then(toml::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    let table = entry.as_table()?;
                    let known_feature = known_section_keys("feature");
                    let unknown_keys = table
                        .keys()
                        .filter(|key| !known_feature.contains(key.as_str()))
                        .cloned()
                        .collect::<Vec<_>>();
                    let name = table
                        .get("name")
                        .or_else(|| table.get("crate"))
                        .and_then(toml::Value::as_str)?
                        .to_owned();
                    Some(FeatureConfigEntry {
                        name,
                        deny: string_array(table.get("deny")),
                        allow: string_array(table.get("allow")),
                        unknown_keys,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

pub fn expected_tokio_allowed_features() -> BTreeSet<String> {
    let parsed = toml::from_str::<toml::Value>(deny::DENY_FEATURE_BANS_TOKIO.content()).ok();
    parsed
        .as_ref()
        .and_then(|value| value.get("bans"))
        .and_then(|value| value.get("features"))
        .and_then(toml::Value::as_array)
        .and_then(|entries| entries.first())
        .map(|entry| string_array(entry.get("allow")))
        .unwrap_or_default()
}

pub fn known_top_level_keys() -> BTreeSet<&'static str> {
    BTreeSet::from(["graph", "bans", "licenses", "advisories", "sources"])
}

pub fn known_section_keys(section: &str) -> BTreeSet<&'static str> {
    match section {
        "graph" => BTreeSet::from(["all-features", "no-default-features"]),
        "bans" => BTreeSet::from([
            "multiple-versions",
            "wildcards",
            "allow-wildcard-paths",
            "highlight",
            "deny",
            "skip",
            "allow",
            "features",
        ]),
        "licenses" => BTreeSet::from(["allow", "confidence-threshold", "private", "exceptions"]),
        "advisories" => BTreeSet::from([
            "unmaintained",
            "yanked",
            "ignore",
            "vulnerability",
            "notice",
            "unsound",
        ]),
        "sources" => BTreeSet::from([
            "unknown-registry",
            "unknown-git",
            "allow-registry",
            "allow-git",
        ]),
        "private" => BTreeSet::from(["ignore"]),
        "exception" => BTreeSet::from(["allow", "name", "crate", "version", "reason"]),
        "skip" => BTreeSet::from(["name", "crate", "version", "reason"]),
        "ignore" => BTreeSet::from(["id", "reason"]),
        "feature" => BTreeSet::from(["name", "crate", "deny", "allow", "reason"]),
        _ => BTreeSet::new(),
    }
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
                    Some((name.to_owned(), string_array(entry.get("wrappers"))))
                })
                .collect()
        })
        .unwrap_or_default()
}
