use super::super::check;
use guardrail3_domain_report::CheckResult;

pub(super) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

pub(super) use ::test_support::{build_fixture_deny_toml, remove_section, set_source_policy};

pub(super) fn expected_sources_for_test() -> (std::collections::BTreeSet<String>, String, String) {
    crate::deny_support::expected_sources()
}
