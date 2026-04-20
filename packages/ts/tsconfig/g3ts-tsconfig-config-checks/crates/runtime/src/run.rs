use g3ts_tsconfig_types::G3TsTsconfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3TsTsconfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_tsconfig_config_01_exists::check(input, &mut results);
    crate::ts_tsconfig_config_02_parseable::check(input, &mut results);
    crate::ts_tsconfig_config_03_extends_chain_resolves::check(input, &mut results);
    crate::ts_tsconfig_config_04_extends_or_inline::check(input, &mut results);
    crate::ts_tsconfig_config_05_strict_baseline::check(input, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
