use super::super::check;
use guardrail3_domain_report::CheckResult;

pub(super) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

pub(super) use ::test_support::{
    add_deny_ban_entry, add_skip_entry, build_fixture_deny_toml, set_advisory_ignores,
    set_feature_entries,
};
