use std::collections::{BTreeMap, BTreeSet};

use guardrail3_domain_modules::deny;

#[derive(Debug, Clone)]
pub struct BanExpectation {
    pub(crate) wrappers: BTreeSet<String>,
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
        return Some(crate_name.split('@').next().unwrap_or(crate_name).to_owned());
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
