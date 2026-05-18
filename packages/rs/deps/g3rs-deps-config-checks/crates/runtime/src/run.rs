use g3rs_deps_types::G3RsDepsConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run extracted dependency config checks for one crate.
#[must_use]
pub fn check(input: &G3RsDepsConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::dependencies_allowlisted::check(input, &mut results);
    crate::build_dependencies_allowlisted::check(input, &mut results);
    crate::dev_dependencies_allowlisted::check(input, &mut results);
    crate::library_allowlist_present::check(input, &mut results);
    crate::direct_dependency_cap::check(input, &mut results);
    crate::cargo_deny_installed::check(input, &mut results);
    crate::cargo_machete_installed::check(input, &mut results);
    crate::cargo_dupes_installed::check(input, &mut results);
    crate::cargo_msrv_verify_installed::check(input, &mut results);
    crate::gitleaks_installed::check(input, &mut results);
    results
}
