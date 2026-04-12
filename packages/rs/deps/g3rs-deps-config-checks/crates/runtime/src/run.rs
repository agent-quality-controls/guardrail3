use g3rs_deps_config_checks_types::G3RsDepsConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run extracted dependency config checks for one crate.
#[must_use]
pub fn check(input: &G3RsDepsConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_deps_config_01_dependencies_allowlisted::rule::check(input, &mut results);
    crate::rs_deps_config_02_build_dependencies_allowlisted::rule::check(input, &mut results);
    crate::rs_deps_config_03_dev_dependencies_allowlisted::rule::check(input, &mut results);
    crate::rs_deps_config_04_library_allowlist_present::rule::check(input, &mut results);
    crate::rs_deps_config_05_direct_dependency_cap::rule::check(input, &mut results);
    crate::rs_deps_config_06_cargo_deny_installed::rule::check(input, &mut results);
    crate::rs_deps_config_07_cargo_machete_installed::rule::check(input, &mut results);
    crate::rs_deps_config_08_cargo_dupes_installed::rule::check(input, &mut results);
    crate::rs_deps_config_09_gitleaks_installed::rule::check(input, &mut results);
    results
}
