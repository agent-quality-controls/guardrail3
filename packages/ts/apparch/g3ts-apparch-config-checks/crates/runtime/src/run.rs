use g3ts_apparch_types::G3TsApparchConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3TsApparchConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_apparch_config_01_types_dependency_direction::check(input, &mut results);
    crate::ts_apparch_config_02_logic_dependency_direction::check(input, &mut results);
    crate::ts_apparch_config_03_io_outbound_dependency_direction::check(input, &mut results);
    crate::ts_apparch_config_04_io_inbound_dependency_direction::check(input, &mut results);
    crate::ts_apparch_config_05_app_no_direct_outbound::check(input, &mut results);
    crate::ts_apparch_config_06_types_purity::check(input, &mut results);
    crate::ts_apparch_config_07_logic_purity::check(input, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
