use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{ban_name, expected_bans, join_set, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        return;
    };
    let allow_entries = bans.get("allow").and_then(toml::Value::as_array);
    let Some(allow_entries) = allow_entries else {
        return;
    };
    let expected = expected_bans(config.profile_name.as_deref());
    let actual_deny = bans
        .get("deny")
        .and_then(toml::Value::as_array)
        .map(|entries| entries.iter().filter_map(ban_name).collect::<BTreeSet<_>>())
        .unwrap_or_default();
    let allow_names = allow_entries
        .iter()
        .filter_map(ban_name)
        .collect::<BTreeSet<_>>();
    if !allow_names.is_empty() {
        results.push(CheckResult {
            id: "RS-DENY-25".to_owned(),
            severity: Severity::Error,
            title: "bans allow-list present".to_owned(),
            message: format!(
                "`{}` has non-empty `[bans].allow`: {}.",
                config.rel_path,
                join_set(&allow_names)
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
    for name in allow_names {
        if expected.contains_key(&name) || actual_deny.contains(&name) {
            results.push(CheckResult {
                id: "RS-DENY-25".to_owned(),
                severity: Severity::Error,
                title: "allow-list overrides deny-list".to_owned(),
                message: format!(
                    "`{}` allows `{name}` even though it is banned.",
                    config.rel_path
                ),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_check_with_profile(deny_toml: &str, profile_name: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, Some(profile_name), check)
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use ::test_support::{
    build_fixture_deny_toml, copy_fixture, set_bans_allow_entries, write_file,
};
#[cfg(test)]
#[path = "rs_deny_25_allow_override_channel_tests/mod.rs"]
// reason: test-only sidecar module wiring
mod rs_deny_25_allow_override_channel_tests;
