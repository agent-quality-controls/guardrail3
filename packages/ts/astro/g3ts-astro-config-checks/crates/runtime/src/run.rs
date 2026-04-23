use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3TsAstroConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_astro_config_01_astro_package_present::check(input, &mut results);
    crate::ts_astro_config_02_astro_check_present::check(input, &mut results);
    crate::ts_astro_config_03_astro_eslint_plugin_package_present::check(input, &mut results);
    crate::ts_astro_config_05_astro_eslint_plugin_wired::check(input, &mut results);
    crate::ts_astro_config_06_pipeline_plugin_package_present::check(input, &mut results);
    crate::ts_astro_config_07_pipeline_plugin_wired::check(input, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
