use g3rs_release_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run all release config checks and return the collected results.
pub fn check(input: &G3RsReleaseConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let repo = input.repo_checks.first();
    for failure in &input.input_failure_checks {
        crate::input_failures::check(failure, &mut results);
    }
    for krate in &input.crate_checks {
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
        crate::binary_release_workflow::check(repo, &input.crate_checks, krate, &mut results);
        crate::linux_release_target::check(repo, &input.crate_checks, krate, &mut results);
    }
    for repo in &input.repo_checks {
        crate::release_plz_baseline::check(repo, &input.crate_checks, &mut results);
        crate::cliff_baseline::check(repo, &input.crate_checks, &mut results);
        crate::semver_checks_installed::check(repo, &input.crate_checks, &mut results);
        crate::publish_status_inventory::check(repo, &input.crate_checks, &mut results);
        crate::release_profile_inventory::check(repo, &input.crate_checks, &mut results);
        crate::crate_inventory::check(repo, &input.crate_checks, &mut results);
    }
    for edge in &input.edge_checks {
        crate::no_path_deps_to_unpublishable::check(edge, &mut results);
        crate::interdependent_version_consistency::check(edge, &mut results);
    }
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
