use g3ts_package_types::G3TsPackageChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsPackageChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::root_exists::check(input, &mut results);
    crate::root_parseable::check(input, &mut results);
    crate::root_private::check(input, &mut results);
    crate::root_package_manager::check(input, &mut results);
    crate::root_engines::check(input, &mut results);
    crate::root_scripts::check(input, &mut results);
    crate::validate_script_present::check(input, &mut results);
    crate::validate_script_fail_closed::check(input, &mut results);
    crate::root_pnpm::check(input, &mut results);
    crate::local_banned_dependencies::check(input, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
