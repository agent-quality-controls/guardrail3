use g3rs_release_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run all release config checks and return the collected results.
#[must_use]
pub fn check(input: &G3RsReleaseConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let first_repo = input.repos.first();
    for failure in &input.input_failures {
        crate::input_failures::check(failure, &mut results);
    }
    for krate in &input.crates {
        crate::publish_must_be_explicit::check(krate, &mut results);
        crate::description_present::check(krate, &mut results);
        crate::license_present::check(krate, &mut results);
        crate::repository_present::check(krate, &mut results);
        crate::keywords_present::check(krate, &mut results);
        crate::categories_present::check(krate, &mut results);
        crate::valid_semver::check(krate, &mut results);
        crate::docs_rs_metadata::check(krate, &mut results);
        crate::binstall_metadata::check(krate, &mut results);
        crate::accidentally_publishable::check(krate, &mut results);
        crate::publish_dry_run::check(krate, &mut results);
        crate::include_exclude_inventory::check(krate, &mut results);
        crate::binary_release_workflow::check(first_repo, &input.crates, krate, &mut results);
        crate::linux_release_target::check(first_repo, &input.crates, krate, &mut results);
    }
    for repo in &input.repos {
        crate::release_plz_baseline::check(repo, &input.crates, &mut results);
        crate::cliff_baseline::check(repo, &input.crates, &mut results);
        crate::semver_checks_installed::check(repo, &input.crates, &mut results);
        crate::publish_status_inventory::check(repo, &input.crates, &mut results);
        crate::release_profile_inventory::check(repo, &input.crates, &mut results);
        crate::crate_inventory::check(repo, &input.crates, &mut results);
    }
    for edge in &input.edges {
        crate::no_path_deps_to_unpublishable::check(edge, &mut results);
        crate::interdependent_version_consistency::check(edge, &mut results);
    }
    results
}
