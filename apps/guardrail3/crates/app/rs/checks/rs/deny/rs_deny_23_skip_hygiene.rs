use std::collections::BTreeMap;

use crate::domain::report::{CheckResult, Severity};

use super::deny_support::section;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        return;
    };
    let Some(skip_entries) = bans.get("skip").and_then(toml::Value::as_array) else {
        return;
    };

    let mut skip_counts = BTreeMap::<String, usize>::new();
    for entry in skip_entries {
        let (name, malformed, non_string_reason, reason) = if let Some(crate_field) =
            entry.get("crate").and_then(toml::Value::as_str)
        {
            let reason_value = entry.get("reason");
            let reason = reason_value.and_then(toml::Value::as_str).map(str::to_owned);
            let name = crate_field.split('@').next().unwrap_or(crate_field).to_owned();
            (name, false, reason_value.is_some() && reason.is_none(), reason)
        } else if let Some(name) = entry.as_str() {
            (name.to_owned(), false, false, None)
        } else if let Some(table) = entry.as_table() {
            let name = table.get("name").and_then(toml::Value::as_str);
            let reason_value = table.get("reason");
            let reason = reason_value.and_then(toml::Value::as_str).map(str::to_owned);
            (
                name.unwrap_or("unknown").to_owned(),
                name.is_none(),
                reason_value.is_some() && reason.is_none(),
                reason,
            )
        } else {
            ("unknown".to_owned(), true, false, None)
        };

        *skip_counts.entry(name.clone()).or_default() += 1;

        if malformed {
            results.push(CheckResult {
                id: "RS-DENY-23".to_owned(),
                severity: Severity::Warn,
                title: "malformed skip entry".to_owned(),
                message: format!(
                    "`{}` has `[bans.skip]` entry without a valid crate identifier.",
                    config.rel_path
                ),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
        if non_string_reason {
            results.push(CheckResult {
                id: "RS-DENY-23".to_owned(),
                severity: Severity::Warn,
                title: "skip reason must be a string".to_owned(),
                message: format!(
                    "`{}` has `[bans.skip]` entry `{name}` with a non-string `reason`.",
                    config.rel_path
                ),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        } else if reason.as_deref().unwrap_or("").trim().is_empty() {
            results.push(CheckResult {
                id: "RS-DENY-23".to_owned(),
                severity: Severity::Warn,
                title: "skip entry missing reason".to_owned(),
                message: format!("`{}` skips `{name}` without a `reason`.", config.rel_path),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }

        if !malformed {
            results.push(
                CheckResult {
                    id: "RS-DENY-23".to_owned(),
                    severity: Severity::Info,
                    title: "skip entry".to_owned(),
                    message: format!("`{}` has skip entry `{name}`.", config.rel_path),
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
#[path = "rs_deny_23_skip_hygiene_tests.rs"]
mod tests;
