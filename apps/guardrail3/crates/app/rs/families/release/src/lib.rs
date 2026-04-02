mod binaries;
mod facts;
mod inputs;
mod publish_integrity;
mod publish_metadata;
mod release_support;
mod repo_inventory;
mod repo_policy;

use guardrail3_app_rs_family_mapper::RsReleaseRoute;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_app_rs_ownership as _;
use guardrail3_domain_config as _;
use guardrail3_domain_modules as _;
use guardrail3_domain_report::CheckResult;
use guardrail3_outbound_traits::ToolChecker;
use proc_macro2 as _;
use quote as _;
use syn as _;

use self::facts::collect;
use self::inputs::{
    PublishableCrateReleaseInput, ReleaseEdgeInput, ReleaseInputFailureInput, RepoReleaseInput,
};

#[cfg(test)]
mod test_fixtures;

pub fn check(
    surface: &FamilyView,
    route: &RsReleaseRoute,
    tc: &dyn ToolChecker,
    thorough: bool,
) -> Vec<CheckResult> {
    let tree = surface;
    let facts = collect(tree, route, tc, thorough);
    let mut results = Vec::new();

    for failure in &facts.input_failures {
        repo_inventory::rs_release_12_input_failures::check(&ReleaseInputFailureInput::new(failure), &mut results);
    }

    for repo in &facts.repo {
        let input = RepoReleaseInput::new(repo);
        repo_policy::rs_release_01_license_file::check(&input, &mut results);
        repo_policy::rs_release_02_release_plz_exists::check(&input, &mut results);
        repo_policy::rs_release_03_release_plz_coverage::check(&input, &mut results);
        repo_policy::rs_release_04_cliff_exists::check(&input, &mut results);
        repo_policy::rs_release_05_release_plz_workflow::check(&input, &mut results);
        repo_policy::rs_release_06_publish_dry_run_workflow::check(&input, &mut results);
        repo_policy::rs_release_07_registry_token::check(&input, &mut results);
        repo_policy::rs_release_08_semver_checks_installed::check(&input, &mut results);
        repo_inventory::rs_release_09_publish_status_inventory::check(&input, &mut results);
        repo_inventory::rs_release_10_release_profile_inventory::check(&input, &mut results);
        publish_integrity::rs_pub_12_crate_inventory::check(&input, &mut results);
    }

    for krate in &facts.crates {
        let input = PublishableCrateReleaseInput::new(krate);
        publish_metadata::rs_pub_01_description_present::check(&input, &mut results);
        publish_metadata::rs_pub_02_license_present::check(&input, &mut results);
        publish_metadata::rs_pub_03_repository_present::check(&input, &mut results);
        publish_metadata::rs_pub_04_readme_exists::check(&input, &mut results);
        publish_metadata::rs_pub_05_readme_quality::check(&input, &mut results);
        publish_metadata::rs_pub_06_keywords_present::check(&input, &mut results);
        publish_metadata::rs_pub_07_categories_present::check(&input, &mut results);
        publish_integrity::rs_pub_08_valid_semver::check(&input, &mut results);
        if thorough {
            publish_integrity::rs_pub_09_publish_dry_run::check(&input, &mut results);
        }
        publish_integrity::rs_pub_13_docs_rs_metadata::check(&input, &mut results);
        publish_integrity::rs_pub_14_include_exclude_inventory::check(&input, &mut results);
        binaries::rs_bin_01_binary_release_workflow::check(&input, &facts.repo, &mut results);
        binaries::rs_bin_02_linux_target::check(&input, &facts.repo, &mut results);
        binaries::rs_bin_03_binstall_metadata::check(&input, &mut results);
        repo_inventory::rs_release_11_accidentally_publishable_internal_crates::check(&input, &mut results);
    }

    for edge in &facts.edges {
        let input = ReleaseEdgeInput::new(edge);
        publish_integrity::rs_pub_10_no_path_deps_to_unpublishable::check(&input, &mut results);
        publish_integrity::rs_pub_11_interdependent_version_consistency::check(&input, &mut results);
    }

    results
}
