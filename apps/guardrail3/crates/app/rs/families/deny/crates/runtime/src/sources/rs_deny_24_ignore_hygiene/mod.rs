use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::validate_reason_text;

use crate::deny_support::section;
use crate::inputs::ConfigDenyInput;

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
    let mut documented_count = 0usize;
    let mut missing_or_invalid_reason_count = 0usize;
    let mut weak_reason_count = 0usize;
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
            missing_or_invalid_reason_count += 1;
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

        let Some(reason) = reason else {
            missing_or_invalid_reason_count += 1;
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
        };
        match validate_reason_text(reason) {
            Ok(()) => {
                documented_count += 1;
            }
            Err(issue) => {
                weak_reason_count += 1;
                results.push(CheckResult::from_parts(
                    "RS-DENY-24".to_owned(),
                    Severity::Error,
                    "advisory ignore reason too weak".to_owned(),
                    format!(
                        "`{}` ignores advisory `{id}` with a weak `reason`: {}.",
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
            id: "RS-DENY-24".to_owned(),
            severity: Severity::Warn,
            title: "advisory ignore entry".to_owned(),
            message: format!(
                "`{}` has documented advisory ignore `{id}`.",
                config.rel_path
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }

    let total = documented_count + missing_or_invalid_reason_count + weak_reason_count;
    if total > 0 {
        results.push(CheckResult::from_parts(
            "RS-DENY-24".to_owned(),
            Severity::Warn,
            "advisory ignore count".to_owned(),
            format!(
                "`{}` has {total} advisory ignores ({documented_count} documented, {missing_or_invalid_reason_count} missing or invalid reasons, {weak_reason_count} weak reasons).",
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
pub(crate) use ::test_support::{build_fixture_deny_toml, set_advisory_ignores};
#[cfg(test)]

mod tests;
