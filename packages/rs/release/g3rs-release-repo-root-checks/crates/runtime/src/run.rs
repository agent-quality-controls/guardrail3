use g3rs_release_types::G3RsReleaseConfigRepo;
use guardrail3_check_types::G3CheckResult;

/// Run all release repo-root checks and return their findings.
#[must_use]
pub fn check(input: &G3RsReleaseConfigRepo) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::release_plz_workflow::check(input, &mut results);
    crate::publish_dry_run_workflow::check(input, &mut results);
    crate::registry_token::check(input, &mut results);
    results
}
