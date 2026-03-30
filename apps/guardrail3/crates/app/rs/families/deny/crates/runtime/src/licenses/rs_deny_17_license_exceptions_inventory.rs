use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::section;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(licenses) = section(config, "licenses") else {
        return;
    };
    if let Some(exceptions) = licenses.get("exceptions").and_then(toml::Value::as_array) {
        for entry in exceptions {
            let Some(table) = entry.as_table() else {
                results.push(CheckResult::from_parts(
                    "RS-DENY-17".to_owned(),
                    Severity::Error,
                    "malformed license exception entry".to_owned(),
                    format!(
                        "`{}` has `[[licenses.exceptions]]` entry without a valid crate identifier.",
                        config.rel_path
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
                continue;
            };

            let Some(name) = table
                .get("name")
                .or_else(|| table.get("crate"))
                .and_then(toml::Value::as_str)
            else {
                results.push(CheckResult::from_parts(
                    "RS-DENY-17".to_owned(),
                    Severity::Error,
                    "malformed license exception entry".to_owned(),
                    format!(
                        "`{}` has `[[licenses.exceptions]]` entry without a valid crate identifier.",
                        config.rel_path
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
                continue;
            };

            let reason_value = table.get("reason");
            let reason = reason_value.and_then(toml::Value::as_str);
            if reason_value.is_some() && reason.is_none() {
                results.push(CheckResult::from_parts(
                    "RS-DENY-17".to_owned(),
                    Severity::Error,
                    "license exception reason must be a string".to_owned(),
                    format!(
                        "`{}` has `[[licenses.exceptions]]` entry `{name}` with a non-string `reason`.",
                        config.rel_path
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
                continue;
            }
            if reason.unwrap_or("").trim().is_empty() {
                results.push(CheckResult::from_parts(
                    "RS-DENY-17".to_owned(),
                    Severity::Error,
                    "license exception missing reason".to_owned(),
                    format!(
                        "`{}` has license exception `{name}` without a `reason`.",
                        config.rel_path
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
                continue;
            }

            results.push(
                CheckResult::from_parts(
                    "RS-DENY-17".to_owned(),
                    Severity::Info,
                    "license exception entry".to_owned(),
                    format!("`{}` has license exception for `{name}`.", config.rel_path),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
    }
}

#[cfg(test)]
pub(crate) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use ::test_support::{
    build_fixture_deny_toml, copy_fixture, set_license_exceptions, write_file,
};
#[cfg(test)]
#[path = "rs_deny_17_license_exceptions_inventory_tests/mod.rs"]
// reason: test-only sidecar module wiring
mod rs_deny_17_license_exceptions_inventory_tests;
