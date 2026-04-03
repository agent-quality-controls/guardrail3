use guardrail3_domain_report::CheckResult;

#[allow(dead_code)]
pub(super) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

pub(super) use crate::config_facts;

pub(super) use ::test_support::{build_fixture_deny_toml, set_advisory_ignores};
