use super::super::check;
use guardrail3_domain_report::CheckResult;

pub(super) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

pub(super) fn run_check_with_profile(deny_toml: &str, profile_name: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, Some(profile_name), check)
}

pub(super) use crate::config_facts_with_profile;

pub(super) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

pub(super) use ::test_support::{
    build_fixture_deny_toml, copy_fixture, remove_deny_ban, set_deny_ban_wrappers, write_file,
};

pub(super) fn expected_ban_names_for_test(
    profile_name: Option<&str>,
) -> std::collections::BTreeSet<String> {
    crate::deny_support::expected_bans(profile_name)
        .into_keys()
        .collect()
}
