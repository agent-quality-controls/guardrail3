use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::validate_reason_text;

use crate::deny_support::section;
use crate::inputs::ConfigDenyInput;

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
    let mut documented_count = 0usize;
    let mut missing_or_invalid_reason_count = 0usize;
    let mut weak_reason_count = 0usize;

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
            missing_or_invalid_reason_count += 1;
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

        let Some(reason) = reason else {
            missing_or_invalid_reason_count += 1;
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
        };
        match validate_reason_text(reason) {
            Ok(()) => {
                documented_count += 1;
            }
            Err(issue) => {
                weak_reason_count += 1;
                results.push(CheckResult::from_parts(
                    "RS-DENY-23".to_owned(),
                    Severity::Error,
                    "skip entry reason too weak".to_owned(),
                    format!(
                        "`{}` skips `{name}` with a weak `reason`: {}.",
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

        results.push(CheckResult {
            id: "RS-DENY-23".to_owned(),
            severity: Severity::Warn,
            title: "skip entry".to_owned(),
            message: format!("`{}` has documented skip entry `{name}`.", config.rel_path),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }

    let total = documented_count + missing_or_invalid_reason_count + weak_reason_count;
    if total > 0 {
        results.push(CheckResult::from_parts(
            "RS-DENY-23".to_owned(),
            Severity::Warn,
            "skip entry count".to_owned(),
            format!(
                "`{}` has {total} skip entries ({documented_count} documented, {missing_or_invalid_reason_count} missing or invalid reasons, {weak_reason_count} weak reasons).",
                config.rel_path
            ),
            None,
            None,
            false,
        ));
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
pub(crate) use ::test_support::{add_skip_entry, build_fixture_deny_toml};
#[cfg(test)]

mod tests;
