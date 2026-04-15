use g3rs_release_repo_root_checks_types::G3RsReleaseRepoRootChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsReleaseRepoRootChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_release_repo_root_01_release_plz_workflow::check(input, &mut results);
    crate::rs_release_repo_root_02_publish_dry_run_workflow::check(input, &mut results);
    crate::rs_release_repo_root_03_registry_token::check(input, &mut results);
    results
}
