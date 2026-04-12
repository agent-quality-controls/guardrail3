use g3rs_toolchain_filetree_checks_types::G3RsToolchainFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsToolchainFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::rs_toolchain_filetree_01_exists::check(input, &mut results);
    crate::rs_toolchain_filetree_04_legacy_file::check(input, &mut results);

    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod tests;
