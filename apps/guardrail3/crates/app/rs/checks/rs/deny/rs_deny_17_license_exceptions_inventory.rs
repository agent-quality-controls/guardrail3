use crate::domain::report::{CheckResult, Severity};

use super::deny_support::section;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(licenses) = section(config, "licenses") else {
        return;
    };
    if let Some(exceptions) = licenses.get("exceptions").and_then(toml::Value::as_array) {
        for entry in exceptions {
            if let Some(name) = entry
                .get("name")
                .or_else(|| entry.get("crate"))
                .and_then(toml::Value::as_str)
            {
                results.push(
                    CheckResult {
                        id: "RS-DENY-17".to_owned(),
                        severity: Severity::Info,
                        title: "license exception entry".to_owned(),
                        message: format!("`{}` has license exception for `{name}`.", config.rel_path),
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
#[path = "rs_deny_17_license_exceptions_inventory_tests.rs"]
mod tests;
