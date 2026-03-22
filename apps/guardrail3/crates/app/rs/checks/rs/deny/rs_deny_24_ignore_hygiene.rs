use crate::domain::report::{CheckResult, Severity};

use super::deny_support::section;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(advisories) = section(config, "advisories") else {
        return;
    };
    if let Some(ignore_entries) = advisories.get("ignore").and_then(toml::Value::as_array) {
        for entry in ignore_entries {
            let (id, reason, malformed, non_string_reason) = if let Some(id) = entry.as_str() {
                (id.to_owned(), None, false, false)
            } else if let Some(table) = entry.as_table() {
                let id = table.get("id").and_then(toml::Value::as_str);
                let reason_value = table.get("reason");
                let reason = reason_value.and_then(toml::Value::as_str).map(str::to_owned);
                let non_string_reason = reason_value.is_some() && reason.is_none();
                (
                    id.unwrap_or("unknown").to_owned(),
                    reason,
                    id.is_none(),
                    non_string_reason,
                )
            } else {
                ("unknown".to_owned(), None, true, false)
            };

            if malformed {
                results.push(CheckResult {
                    id: "RS-DENY-24".to_owned(),
                    severity: Severity::Warn,
                    title: "malformed advisory ignore entry".to_owned(),
                    message: format!(
                        "`{}` has an `[advisories].ignore` entry without a valid advisory id.",
                        config.rel_path
                    ),
                    file: Some(config.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
            if non_string_reason {
                results.push(CheckResult {
                    id: "RS-DENY-24".to_owned(),
                    severity: Severity::Warn,
                    title: "advisory ignore reason must be a string".to_owned(),
                    message: format!(
                        "`{}` has `[advisories].ignore` entry `{id}` with a non-string `reason`.",
                        config.rel_path
                    ),
                    file: Some(config.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            } else if reason.as_deref().unwrap_or("").trim().is_empty() {
                results.push(CheckResult {
                    id: "RS-DENY-24".to_owned(),
                    severity: Severity::Warn,
                    title: "advisory ignore missing reason".to_owned(),
                    message: format!(
                        "`{}` ignores advisory `{id}` without a `reason`.",
                        config.rel_path
                    ),
                    file: Some(config.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
            if !malformed && !non_string_reason && !reason.as_deref().unwrap_or("").trim().is_empty() {
                results.push(
                    CheckResult {
                        id: "RS-DENY-24".to_owned(),
                        severity: Severity::Info,
                        title: "advisory ignore entry".to_owned(),
                        message: format!("`{}` ignores advisory `{id}`.", config.rel_path),
                        file: Some(config.rel_path.clone()),
                        line: None,
                        inventory: false,
                    }
                    .as_inventory(),
                );
            }
        }
    }
}

#[cfg(test)]
#[path = "rs_deny_24_ignore_hygiene_tests.rs"]
mod tests;
