use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3TsEslintConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_eslint_config_01_exists::check(input, &mut results);
    crate::ts_eslint_config_02_parseable::check(input, &mut results);
    crate::ts_eslint_config_03_ts_plugin_present::check(input, &mut results);
    crate::ts_eslint_config_04_project_service_enabled::check(input, &mut results);
    crate::ts_eslint_config_05_no_explicit_any_error::check(input, &mut results);
    crate::ts_eslint_config_06_no_console_error::check(input, &mut results);
    crate::ts_eslint_config_07_thresholds::check(input, &mut results);
    crate::ts_eslint_config_08_core_baseline_rules::check(input, &mut results);
    crate::ts_eslint_config_09_type_safety_rules::check(input, &mut results);
    crate::ts_eslint_config_10_hygiene_rules::check(input, &mut results);
    crate::ts_eslint_config_11_unicorn_rules::check(input, &mut results);
    crate::ts_eslint_config_12_regexp_rules::check(input, &mut results);
    crate::ts_eslint_config_13_sonarjs_rules::check(input, &mut results);
    crate::ts_eslint_config_14_test_relaxations::check(input, &mut results);
    crate::ts_eslint_config_15_js_carveout::check(input, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
// reason: owned sidecar tests for file module.
mod run_tests;
