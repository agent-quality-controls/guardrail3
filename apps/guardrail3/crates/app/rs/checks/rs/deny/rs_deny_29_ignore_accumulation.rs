use crate::domain::report::{CheckResult, Severity};

use super::deny_support::section;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(advisories) = section(config, "advisories") else {
        return;
    };
    let Some(ignore_entries) = advisories.get("ignore").and_then(toml::Value::as_array) else {
        return;
    };
    if ignore_entries.len() > 5 {
        results.push(CheckResult {
            id: "RS-DENY-29".to_owned(),
            severity: Severity::Warn,
            title: "advisory ignore list is large".to_owned(),
            message: format!(
                "`{}` has {} `[advisories].ignore` entries (threshold: 5).",
                config.rel_path,
                ignore_entries.len()
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_deny_29_ignore_accumulation_tests.rs"]
mod tests;
