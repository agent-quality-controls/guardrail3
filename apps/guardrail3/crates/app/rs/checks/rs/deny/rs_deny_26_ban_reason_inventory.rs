use crate::domain::report::{CheckResult, Severity};

use super::deny_support::section;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        return;
    };
    let Some(deny_entries) = bans.get("deny").and_then(toml::Value::as_array) else {
        return;
    };
    for entry in deny_entries {
        let Some(table) = entry.as_table() else {
            continue;
        };
        let Some(name) = table.get("name").and_then(toml::Value::as_str) else {
            continue;
        };
        if table.get("reason").and_then(toml::Value::as_str).unwrap_or("").trim().is_empty() {
            results.push(
                CheckResult {
                    id: "RS-DENY-26".to_owned(),
                    severity: Severity::Info,
                    title: "ban entry missing reason".to_owned(),
                    message: format!("`{}` ban entry `{name}` has no `reason`.", config.rel_path),
                    file: Some(config.rel_path.clone()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        }
    }
}

#[cfg(test)]
#[path = "rs_deny_26_ban_reason_inventory_tests.rs"]
mod tests;
