use std::collections::BTreeMap;

use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::{ban_name, parse_feature_entries_in_config, section};
use crate::inputs::ConfigDenyInput;

fn normalized_skip_identity(table: &toml::map::Map<String, toml::Value>) -> Option<String> {
    if let Some(crate_field) = table.get("crate").and_then(toml::Value::as_str) {
        let crate_field = crate_field.trim();
        if crate_field.is_empty() {
            return None;
        }

        if let Some((name, version)) = crate_field.split_once('@') {
            let name = name.trim();
            let version = version.trim();
            if name.is_empty() || version.is_empty() {
                return None;
            }
            return Some(format!("{name}@{version}"));
        }

        let version = table
            .get("version")
            .and_then(toml::Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty());
        return Some(match version {
            Some(version) => format!("{crate_field}@{version}"),
            None => crate_field.to_owned(),
        });
    }

    let name = table
        .get("name")
        .and_then(toml::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())?;
    let version = table
        .get("version")
        .and_then(toml::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty());
    Some(match version {
        Some(version) => format!("{name}@{version}"),
        None => name.to_owned(),
    })
}

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(parsed) = &config.parsed else {
        return;
    };

    if let Some(bans) = section(config, "bans") {
        let mut deny_counts = BTreeMap::<String, usize>::new();
        if let Some(entries) = bans.get("deny").and_then(toml::Value::as_array) {
            for entry in entries {
                if let Some(name) = ban_name(entry) {
                    *deny_counts.entry(name).or_default() += 1;
                }
            }
        }
        for (name, count) in deny_counts {
            if count > 1 {
                results.push(CheckResult::from_parts(
                    "RS-DENY-27".to_owned(),
                    Severity::Warn,
                    "duplicate deny entry".to_owned(),
                    format!("`{}` has duplicate deny entry `{name}`.", config.rel_path),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
            }
        }

        let mut skip_counts = BTreeMap::<String, usize>::new();
        if let Some(entries) = bans.get("skip").and_then(toml::Value::as_array) {
            for entry in entries {
                let Some(table) = entry.as_table() else {
                    continue;
                };
                let Some(name) = normalized_skip_identity(table) else {
                    continue;
                };
                *skip_counts.entry(name).or_default() += 1;
            }
        }
        for (name, count) in skip_counts {
            if count > 1 {
                results.push(CheckResult::from_parts(
                    "RS-DENY-27".to_owned(),
                    Severity::Warn,
                    "duplicate skip entry".to_owned(),
                    format!("`{}` has duplicate skip entry `{name}`.", config.rel_path),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
            }
        }
    }

    let mut ignore_counts = BTreeMap::<String, usize>::new();
    if let Some(advisories) = section(config, "advisories") {
        if let Some(entries) = advisories.get("ignore").and_then(toml::Value::as_array) {
            for entry in entries {
                let Some(table) = entry.as_table() else {
                    continue;
                };
                let Some(id) = table
                    .get("id")
                    .and_then(toml::Value::as_str)
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                else {
                    continue;
                };
                *ignore_counts.entry(id.to_owned()).or_default() += 1;
            }
        }
    }
    for (id, count) in ignore_counts {
        if count > 1 {
            results.push(CheckResult::from_parts(
                "RS-DENY-27".to_owned(),
                Severity::Warn,
                "duplicate advisory ignore entry".to_owned(),
                format!(
                    "`{}` has duplicate advisory ignore `{id}`.",
                    config.rel_path
                ),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
        }
    }

    let mut feature_counts = BTreeMap::<String, usize>::new();
    for entry in parse_feature_entries_in_config(parsed) {
        *feature_counts.entry(entry.name).or_default() += 1;
    }
    for (name, count) in feature_counts {
        if count > 1 {
            results.push(CheckResult::from_parts(
                "RS-DENY-27".to_owned(),
                Severity::Warn,
                "duplicate feature-ban entry".to_owned(),
                format!(
                    "`{}` has duplicate `[[bans.features]]` for `{name}`.",
                    config.rel_path
                ),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
        }
    }
}

#[cfg(test)]
pub(crate) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[cfg(test)]
pub(crate) use ::test_support::{
    add_deny_ban_entry, add_skip_entry, build_fixture_deny_toml, set_advisory_ignores,
    set_feature_entries,
};
#[cfg(test)]
#[path = "rs_deny_27_duplicate_entries_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_27_duplicate_entries_tests;
