use g3ts_arch_types::G3TsArchSourceChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3TsArchSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_arch_source_01_facade_parseable::check(input, &mut results);
    crate::ts_arch_source_02_facade_only::check(input, &mut results);
    crate::ts_arch_source_03_no_broad_reexport::check(input, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
