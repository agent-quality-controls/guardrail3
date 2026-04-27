use g3ts_arch_types::G3TsArchConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3TsArchConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::root_manifest_exists::check(input, &mut results);
    crate::root_manifest_parseable::check(input, &mut results);
    crate::declared_entrypoints_canonical::check(input, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
