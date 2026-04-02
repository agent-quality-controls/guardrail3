use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::{ban_name, expected_bans, join_set, section};
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    if !config.policy_context_valid {
        return;
    }
    let Some(bans) = section(config, "bans") else {
        return;
    };
    let Some(allow_value) = bans.get("allow") else {
        return;
    };
    let Some(allow_entries) = allow_value.as_array() else {
        push_malformed_allow_error(
            config,
            format!(
                "`{}` must keep `[bans].allow` as an array of crate entries.",
                config.rel_path
            ),
            results,
        );
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
    if allow_names.len() != allow_entries.len() {
        push_malformed_allow_error(
            config,
            format!(
                "`{}` has malformed `[bans].allow` entries that cannot be matched to crate names.",
                config.rel_path
            ),
            results,
        );
    }
    if !allow_names.is_empty() {
        results.push(CheckResult::from_parts(
            "RS-DENY-25".to_owned(),
            Severity::Error,
            "bans allow-list present".to_owned(),
            format!(
                "`{}` has non-empty `[bans].allow`: {}.",
                config.rel_path,
                join_set(&allow_names)
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    }
    for name in allow_names {
        if expected.contains_key(&name) || actual_deny.contains(&name) {
            results.push(CheckResult::from_parts(
                "RS-DENY-25".to_owned(),
                Severity::Error,
                "allow-list overrides deny-list".to_owned(),
                format!(
                    "`{}` allows `{name}` even though it is banned.",
                    config.rel_path
                ),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
        }
    }
}

fn push_malformed_allow_error(
    config: &crate::facts::DenyConfigFacts,
    message: String,
    results: &mut Vec<CheckResult>,
) {
    results.push(CheckResult::from_parts(
        "RS-DENY-25".to_owned(),
        Severity::Error,
        "bans allow-list malformed".to_owned(),
        message,
        Some(config.rel_path.clone()),
        None,
        false,
    ));
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
pub(crate) use ::test_support::{build_fixture_deny_toml, set_bans_allow_entries};
#[cfg(test)]

// reason: test-only sidecar module wiring
mod rs_deny_25_allow_override_channel_tests;
