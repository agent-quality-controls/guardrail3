use g3rs_clippy_types::G3RsClippyFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsClippyFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::rs_clippy_filetree_01_coverage_exists::check(input, &mut results);
    crate::rs_clippy_filetree_02_same_root_conflict::check(input, &mut results);

    results
}
