use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{
    expected_tokio_allowed_features, join_set, parse_feature_entries_in_config,
};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(parsed) = &config.parsed else {
        return;
    };
    let feature_entries = parse_feature_entries_in_config(parsed);
    let tokio_entries = feature_entries
        .iter()
        .filter(|entry| entry.name == "tokio")
        .collect::<Vec<_>>();
    if tokio_entries.is_empty() {
        results.push(CheckResult::from_parts(
            "RS-DENY-21".to_owned(),
            Severity::Warn,
            "tokio full feature not banned".to_owned(),
            format!(
                "`{}` must ban `tokio` feature `full` under `[[bans.features]]`.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    if tokio_entries
        .iter()
        .any(|entry| !entry.deny.contains("full"))
    {
        results.push(CheckResult::from_parts(
            "RS-DENY-21".to_owned(),
            Severity::Warn,
            "tokio full feature not banned".to_owned(),
            format!(
                "`{}` must ban `tokio` feature `full` under `[[bans.features]]`.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    }

    let expected_allow = expected_tokio_allowed_features();
    if tokio_entries
        .iter()
        .any(|entry| entry.allow != expected_allow)
    {
        results.push(CheckResult::from_parts(
            "RS-DENY-21".to_owned(),
            Severity::Warn,
            "tokio allowed features changed".to_owned(),
            format!(
                "`{}` must keep `tokio` allowed features `{}`.",
                config.rel_path,
                join_set(&expected_allow)
            ),
            Some(config.rel_path.clone()),
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
    build_fixture_deny_toml, copy_fixture, set_feature_entries, write_file,
};
#[cfg(test)]
pub(crate) fn expected_tokio_allowed_features_for_test() -> std::collections::BTreeSet<String> {
    super::deny_support::expected_tokio_allowed_features()
}
#[cfg(test)]
pub(crate) fn join_set_for_test(values: &std::collections::BTreeSet<String>) -> String {
    super::deny_support::join_set(values)
}
#[cfg(test)]
pub(crate) fn parse_feature_entries_for_test(
    parsed: &toml::Value,
) -> Vec<super::deny_support::FeatureConfigEntry> {
    super::deny_support::parse_feature_entries_in_config(parsed)
}
#[cfg(test)]
#[path = "rs_deny_21_tokio_full_ban_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_21_tokio_full_ban_tests;
