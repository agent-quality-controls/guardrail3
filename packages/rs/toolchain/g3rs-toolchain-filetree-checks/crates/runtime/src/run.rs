use g3rs_toolchain_types::G3RsToolchainFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3RsToolchainFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::exists::check(input, &mut results);
    crate::legacy_file::check(input, &mut results);

    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
