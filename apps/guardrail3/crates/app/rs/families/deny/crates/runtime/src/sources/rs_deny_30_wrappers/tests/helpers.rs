use super::super::check;
use guardrail3_domain_report::CheckResult;

pub(super) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

pub(super) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

pub(super) use ::test_support::{
    add_deny_ban_entry, build_fixture_deny_toml, copy_fixture, set_deny_ban_wrappers, write_file,
};

pub(super) fn expected_ban_wrappers_for_test(
    profile_name: Option<&str>,
) -> std::collections::BTreeMap<String, std::collections::BTreeSet<String>> {
    crate::deny_support::expected_bans(profile_name)
        .into_iter()
        .map(|(name, expectation)| (name, expectation.wrappers))
        .collect()
}
