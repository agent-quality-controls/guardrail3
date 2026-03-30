use std::collections::BTreeMap;

use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::section;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        return;
    };
    let Some(skip_value) = bans.get("skip") else {
        return;
    };
    let Some(skip_entries) = skip_value.as_array() else {
        results.push(CheckResult {
            id: "RS-DENY-23".to_owned(),
            severity: Severity::Warn,
            title: "malformed skip container".to_owned(),
            message: format!(
                "`{}` must use an array for `[bans].skip` entries.",
                config.rel_path
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    };

    let mut skip_counts = BTreeMap::<String, usize>::new();
    for entry in skip_entries {
        let (name, malformed, non_string_reason, reason, plain_string_entry) =
            if let Some(crate_field) = entry.get("crate").and_then(toml::Value::as_str) {
                let reason_value = entry.get("reason");
                let reason = reason_value
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned);
                let name = crate_field
                    .split('@')
                    .next()
                    .unwrap_or(crate_field)
                    .to_owned();
                (
                    name,
                    false,
                    reason_value.is_some() && reason.is_none(),
                    reason,
                    false,
                )
            } else if let Some(name) = entry.as_str() {
                (name.to_owned(), false, false, None, true)
            } else if let Some(table) = entry.as_table() {
                let name = table.get("name").and_then(toml::Value::as_str);
                let reason_value = table.get("reason");
                let reason = reason_value
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned);
                (
                    name.unwrap_or("unknown").to_owned(),
                    name.is_none(),
                    reason_value.is_some() && reason.is_none(),
                    reason,
                    false,
                )
            } else {
                ("unknown".to_owned(), true, false, None, false)
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
        } else if !plain_string_entry && reason.as_deref().unwrap_or("").trim().is_empty() {
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

        if !malformed
            && !non_string_reason
            && (plain_string_entry || !reason.as_deref().unwrap_or("").trim().is_empty())
        {
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
pub(crate) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use ::test_support::{
    add_skip_entry, build_fixture_deny_toml, copy_fixture, write_file,
};
#[cfg(test)]
#[cfg(test)]
#[path = "rs_deny_23_skip_hygiene_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_23_skip_hygiene_tests;
