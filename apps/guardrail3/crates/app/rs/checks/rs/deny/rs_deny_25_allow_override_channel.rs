use std::collections::BTreeSet;

use crate::domain::report::{CheckResult, Severity};

use super::deny_support::{ban_name, expected_bans, join_set, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        return;
    };
    let allow_entries = bans.get("allow").and_then(toml::Value::as_array);
    let Some(allow_entries) = allow_entries else {
        return;
    };
    let expected = expected_bans(config.profile_name.as_deref());
    let actual_deny = bans
        .get("deny")
        .and_then(toml::Value::as_array)
        .map(|entries| entries.iter().filter_map(ban_name).collect::<BTreeSet<_>>())
        .unwrap_or_default();
    let allow_names = allow_entries
        .iter()
        .filter_map(ban_name)
        .collect::<BTreeSet<_>>();
    if !allow_names.is_empty() {
        results.push(CheckResult {
            id: "RS-DENY-25".to_owned(),
            severity: Severity::Error,
            title: "bans allow-list present".to_owned(),
            message: format!(
                "`{}` has non-empty `[bans].allow`: {}.",
                config.rel_path,
                join_set(&allow_names)
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
    for name in allow_names {
        if expected.contains_key(&name) || actual_deny.contains(&name) {
            results.push(CheckResult {
                id: "RS-DENY-25".to_owned(),
                severity: Severity::Error,
                title: "allow-list overrides deny-list".to_owned(),
                message: format!(
                    "`{}` allows `{name}` even though it is banned.",
                    config.rel_path
                ),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[cfg(test)]
#[path = "rs_deny_25_allow_override_channel_tests.rs"]
mod tests;
