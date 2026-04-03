use super::super::check;
use guardrail3_domain_report::CheckResult;

pub(super) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

pub(super) use ::test_support::{
    add_allowed_license, build_fixture_deny_toml, remove_allowed_license, remove_section,
    set_private_ignore,
};

pub(super) fn expected_licenses_for_test() -> std::collections::BTreeSet<String> {
    crate::deny_support::expected_licenses()
}
