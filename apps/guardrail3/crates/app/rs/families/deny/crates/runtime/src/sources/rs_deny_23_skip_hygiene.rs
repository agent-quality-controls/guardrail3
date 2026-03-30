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
        results.push(CheckResult::from_parts(
            "RS-DENY-23".to_owned(),
            Severity::Error,
            "malformed skip container".to_owned(),
            format!(
                "`{}` must use an array for `[bans].skip` entries.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    };

    for entry in skip_entries {
        let Some(table) = entry.as_table() else {
            if let Some(name) = entry.as_str() {
                results.push(CheckResult::from_parts(
                    "RS-DENY-23".to_owned(),
                    Severity::Error,
                    "skip entry must use table form".to_owned(),
                    format!(
                        "`{}` has `[bans.skip]` string entry `{name}`; use table form with a `reason`.",
                        config.rel_path
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
            } else {
                results.push(CheckResult::from_parts(
                    "RS-DENY-23".to_owned(),
                    Severity::Error,
                    "malformed skip entry".to_owned(),
                    format!(
                        "`{}` has `[bans.skip]` entry without a valid crate identifier.",
                        config.rel_path
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
            }
            continue;
        };

        let name = if let Some(crate_field) = table.get("crate").and_then(toml::Value::as_str) {
            crate_field
                .split('@')
                .next()
                .unwrap_or(crate_field)
                .to_owned()
        } else if let Some(name) = table.get("name").and_then(toml::Value::as_str) {
            name.to_owned()
        } else {
            results.push(CheckResult::from_parts(
                "RS-DENY-23".to_owned(),
                Severity::Error,
                "malformed skip entry".to_owned(),
                format!(
                    "`{}` has `[bans.skip]` entry without a valid crate identifier.",
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
                "RS-DENY-23".to_owned(),
                Severity::Error,
                "skip reason must be a string".to_owned(),
                format!(
                    "`{}` has `[bans.skip]` entry `{name}` with a non-string `reason`.",
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
                "RS-DENY-23".to_owned(),
                Severity::Error,
                "skip entry missing reason".to_owned(),
                format!("`{}` skips `{name}` without a `reason`.", config.rel_path),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
            continue;
        }

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
#[path = "rs_deny_23_skip_hygiene_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_23_skip_hygiene_tests;
