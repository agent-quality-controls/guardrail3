use super::super::check;
pub(super) fn run_check(deny_toml: &str) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[allow(dead_code)]
pub(super) fn run_family(root: &std::path::Path) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check_test_root(root)
}

pub(super) use ::test_support::{build_fixture_deny_toml, set_section_string};
