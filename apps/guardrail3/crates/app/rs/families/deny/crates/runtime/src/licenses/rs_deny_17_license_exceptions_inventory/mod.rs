use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::validate_reason_text;

use crate::deny_support::section;
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(licenses) = section(config, "licenses") else {
        return;
    };
    if let Some(exceptions) = licenses.get("exceptions").and_then(toml::Value::as_array) {
        let mut documented_count = 0usize;
        let mut missing_or_invalid_reason_count = 0usize;
        let mut weak_reason_count = 0usize;
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
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
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

            if table
                .get("allow")
                .and_then(toml::Value::as_array)
                .is_some_and(|entries| {
                    entries
                        .iter()
                        .filter_map(toml::Value::as_str)
                        .any(|entry| entry.trim().is_empty())
                })
            {
                results.push(CheckResult::from_parts(
                    "RS-DENY-17".to_owned(),
                    Severity::Error,
                    "malformed license exception entry".to_owned(),
                    format!(
                        "`{}` has `[[licenses.exceptions]]` entry `{name}` with blank allowed license name.",
                        config.rel_path
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
                continue;
            }

            let reason_value = table.get("reason");
            let reason = reason_value.and_then(toml::Value::as_str);
            if reason_value.is_some() && reason.is_none() {
                missing_or_invalid_reason_count += 1;
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
            let Some(reason) = reason else {
                missing_or_invalid_reason_count += 1;
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
            };
            match validate_reason_text(reason) {
                Ok(()) => {
                    documented_count += 1;
                }
                Err(issue) => {
                    weak_reason_count += 1;
                    results.push(CheckResult::from_parts(
                        "RS-DENY-17".to_owned(),
                        Severity::Error,
                        "license exception reason too weak".to_owned(),
                        format!(
                            "`{}` has license exception `{name}` with a weak `reason`: {}.",
                            config.rel_path,
                            issue.message()
                        ),
                        Some(config.rel_path.clone()),
                        None,
                        false,
                    ));
                    continue;
                }
            }

            results.push(CheckResult::from_parts(
                "RS-DENY-17".to_owned(),
                Severity::Warn,
                "license exception entry".to_owned(),
                format!(
                    "`{}` has documented license exception for `{name}`.",
                    config.rel_path
                ),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
        }

        let total = documented_count + missing_or_invalid_reason_count + weak_reason_count;
        if total > 0 {
            results.push(CheckResult::from_parts(
                "RS-DENY-17".to_owned(),
                Severity::Warn,
                "license exception count".to_owned(),
                format!(
                    "`{}` has {total} license exceptions ({documented_count} documented, {missing_or_invalid_reason_count} missing or invalid reasons, {weak_reason_count} weak reasons).",
                    config.rel_path
                ),
                None,
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
#[allow(dead_code)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use ::test_support::{build_fixture_deny_toml, set_license_exceptions};
#[cfg(test)]

// reason: test-only sidecar module wiring
mod tests;
