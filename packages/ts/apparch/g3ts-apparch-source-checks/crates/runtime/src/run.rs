use g3ts_apparch_types::G3TsApparchSourceChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsApparchSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::types_public_surface::check(input, &mut results);
    crate::io_contracts_in_types::check(input, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
