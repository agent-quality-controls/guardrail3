use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::validate_reason_text;

use super::deny_support::{ban_name, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        return;
    };
    let Some(deny_entries) = bans.get("deny").and_then(toml::Value::as_array) else {
        return;
    };
    let mut documented_count = 0usize;
    let mut missing_reason_count = 0usize;
    let mut weak_reason_count = 0usize;
    for entry in deny_entries {
        let Some(name) = ban_name(entry) else {
            continue;
        };
        let reason = entry
            .as_table()
            .and_then(|table| table.get("reason"))
            .and_then(toml::Value::as_str)
            .unwrap_or("");
        if reason.trim().is_empty() {
            missing_reason_count += 1;
            results.push(CheckResult::from_parts(
                "RS-DENY-26".to_owned(),
                Severity::Error,
                "ban entry missing reason".to_owned(),
                format!("`{}` ban entry `{name}` has no `reason`.", config.rel_path),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
            continue;
        }

        match validate_reason_text(reason) {
            Ok(()) => {
                documented_count += 1;
                results.push(CheckResult::from_parts(
                    "RS-DENY-26".to_owned(),
                    Severity::Warn,
                    "ban entry".to_owned(),
                    format!("`{}` has documented ban entry `{name}`.", config.rel_path),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
            }
            Err(issue) => {
                weak_reason_count += 1;
                results.push(CheckResult::from_parts(
                    "RS-DENY-26".to_owned(),
                    Severity::Error,
                    "ban entry reason too weak".to_owned(),
                    format!(
                        "`{}` ban entry `{name}` has a weak `reason`: {}.",
                        config.rel_path,
                        issue.message()
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
            }
        }
    }

    let total = documented_count + missing_reason_count + weak_reason_count;
    if total > 0 {
        results.push(CheckResult::from_parts(
            "RS-DENY-26".to_owned(),
            Severity::Warn,
            "ban entry count".to_owned(),
            format!(
                "`{}` has {total} deny ban entries ({documented_count} documented, {missing_reason_count} missing reasons, {weak_reason_count} weak reasons).",
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
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use ::test_support::{
    add_deny_ban_entry, build_fixture_deny_toml, copy_fixture, remove_deny_ban_reason, write_file,
};
#[cfg(test)]
#[path = "rs_deny_26_ban_reason_inventory_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_26_ban_reason_inventory_tests;
