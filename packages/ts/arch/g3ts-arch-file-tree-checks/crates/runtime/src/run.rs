use g3ts_arch_types::G3TsArchFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3TsArchFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::declared_entrypoint_exists::check(input, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
