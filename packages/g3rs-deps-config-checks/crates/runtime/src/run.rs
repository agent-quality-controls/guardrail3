use g3rs_deps_config_checks_types::{G3RsDepsConfigDirectDependencyCapInput, G3RsDepsConfigPolicyChecksInput};
use guardrail3_check_types::G3CheckResult;

pub fn check_policy(input: &G3RsDepsConfigPolicyChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_deps_config_01_dependencies_allowlisted::rule::check(input, &mut results);
    crate::rs_deps_config_02_build_dependencies_allowlisted::rule::check(input, &mut results);
    crate::rs_deps_config_03_dev_dependencies_allowlisted::rule::check(input, &mut results);
    crate::rs_deps_config_04_library_allowlist_present::rule::check(input, &mut results);
    results
}

pub fn check_direct_dependency_cap(input: &G3RsDepsConfigDirectDependencyCapInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_deps_config_05_direct_dependency_cap::rule::check(input, &mut results);
    results
}
