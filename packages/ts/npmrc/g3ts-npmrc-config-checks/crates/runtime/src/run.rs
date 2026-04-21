use g3ts_npmrc_types::G3TsNpmrcChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3TsNpmrcChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_npmrc_config_01_root_exists::check(input, &mut results);
    crate::ts_npmrc_config_02_root_parseable::check(input, &mut results);
    crate::ts_npmrc_config_03_duplicate_keys::check(input, &mut results);
    crate::ts_npmrc_config_04_required_settings_present::check(input, &mut results);
    crate::ts_npmrc_config_05_required_settings_strong_enough::check(input, &mut results);
    crate::ts_npmrc_config_06_extra_settings_inventory::check(input, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
