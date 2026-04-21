use g3ts_jscpd_types::G3TsJscpdChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3TsJscpdChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_jscpd_config_01_root_exists::check(input, &mut results);
    crate::ts_jscpd_config_02_root_parseable::check(input, &mut results);
    crate::ts_jscpd_config_03_threshold_zero::check(input, &mut results);
    crate::ts_jscpd_config_04_absolute_true::check(input, &mut results);
    crate::ts_jscpd_config_05_required_ignores::check(input, &mut results);
    crate::ts_jscpd_config_06_format_and_inventory::check(input, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
