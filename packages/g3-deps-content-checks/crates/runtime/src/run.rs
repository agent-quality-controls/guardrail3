use g3_deps_content_checks_types::{G3DepsDirectDependencyCapInput, G3DepsPolicyContentChecksInput};
use guardrail3_check_types::G3CheckResult;

pub fn check_policy(input: &G3DepsPolicyContentChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_deps_05_dependencies_allowlisted::rule::check(input, &mut results);
    crate::rs_deps_06_build_dependencies_allowlisted::rule::check(input, &mut results);
    crate::rs_deps_07_dev_dependencies_allowlisted::rule::check(input, &mut results);
    crate::rs_deps_08_library_allowlist_present::rule::check(input, &mut results);
    results
}

pub fn check_direct_dependency_cap(input: &G3DepsDirectDependencyCapInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_deps_12_direct_dependency_cap::rule::check(input, &mut results);
    results
}
