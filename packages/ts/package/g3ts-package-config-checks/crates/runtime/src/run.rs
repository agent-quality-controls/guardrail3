use g3ts_package_types::G3TsPackageChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3TsPackageChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_package_config_01_root_exists::check(input, &mut results);
    crate::ts_package_config_02_root_parseable::check(input, &mut results);
    crate::ts_package_config_03_root_private::check(input, &mut results);
    crate::ts_package_config_04_root_package_manager::check(input, &mut results);
    crate::ts_package_config_05_root_engines::check(input, &mut results);
    crate::ts_package_config_06_root_scripts::check(input, &mut results);
    crate::ts_package_config_07_root_pnpm::check(input, &mut results);
    crate::ts_package_config_08_local_banned_dependencies::check(input, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
