use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::section;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(advisories) = section(config, "advisories") else {
        return;
    };
    let Some(ignore_value) = advisories.get("ignore") else {
        return;
    };
    let Some(ignore_entries) = ignore_value.as_array() else {
        results.push(CheckResult::from_parts(
            "RS-DENY-24".to_owned(),
            Severity::Error,
            "malformed advisory ignore container".to_owned(),
            format!(
                "`{}` must use an array for `[advisories].ignore` entries.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    };
    for entry in ignore_entries {
        let Some(table) = entry.as_table() else {
            if let Some(id) = entry.as_str() {
                results.push(CheckResult::from_parts(
                    "RS-DENY-24".to_owned(),
                    Severity::Error,
                    "advisory ignore must use table form".to_owned(),
                    format!(
                        "`{}` has `[advisories].ignore` string entry `{id}`; use table form with a `reason`.",
                        config.rel_path
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
            } else {
                results.push(CheckResult::from_parts(
                    "RS-DENY-24".to_owned(),
                    Severity::Error,
                    "malformed advisory ignore entry".to_owned(),
                    format!(
                        "`{}` has an `[advisories].ignore` entry without a valid advisory id.",
                        config.rel_path
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
            }
            continue;
        };

        let Some(id) = table.get("id").and_then(toml::Value::as_str) else {
            results.push(CheckResult::from_parts(
                "RS-DENY-24".to_owned(),
                Severity::Error,
                "malformed advisory ignore entry".to_owned(),
                format!(
                    "`{}` has an `[advisories].ignore` entry without a valid advisory id.",
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
                "RS-DENY-24".to_owned(),
                Severity::Error,
                "advisory ignore reason must be a string".to_owned(),
                format!(
                    "`{}` has `[advisories].ignore` entry `{id}` with a non-string `reason`.",
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
                "RS-DENY-24".to_owned(),
                Severity::Error,
                "advisory ignore missing reason".to_owned(),
                format!(
                    "`{}` ignores advisory `{id}` without a `reason`.",
                    config.rel_path
                ),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
            continue;
        }

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
    build_fixture_deny_toml, copy_fixture, set_advisory_ignores, write_file,
};
#[cfg(test)]
#[path = "rs_deny_24_ignore_hygiene_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_24_ignore_hygiene_tests;
