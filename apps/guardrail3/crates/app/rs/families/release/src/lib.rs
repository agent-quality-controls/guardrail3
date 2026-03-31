mod facts;
mod inputs;
mod release_support;
#[path = "binaries/rs_bin_01_binary_release_workflow.rs"]
mod rs_bin_01_binary_release_workflow;
#[path = "binaries/rs_bin_02_linux_target.rs"]
mod rs_bin_02_linux_target;
#[path = "binaries/rs_bin_03_binstall_metadata.rs"]
mod rs_bin_03_binstall_metadata;
#[path = "publish_metadata/rs_pub_01_description_present.rs"]
mod rs_pub_01_description_present;
#[path = "publish_metadata/rs_pub_02_license_present.rs"]
mod rs_pub_02_license_present;
#[path = "publish_metadata/rs_pub_03_repository_present.rs"]
mod rs_pub_03_repository_present;
#[path = "publish_metadata/rs_pub_04_readme_exists.rs"]
mod rs_pub_04_readme_exists;
#[path = "publish_metadata/rs_pub_05_readme_quality.rs"]
mod rs_pub_05_readme_quality;
#[path = "publish_metadata/rs_pub_06_keywords_present.rs"]
mod rs_pub_06_keywords_present;
#[path = "publish_metadata/rs_pub_07_categories_present.rs"]
mod rs_pub_07_categories_present;
#[path = "publish_integrity/rs_pub_08_valid_semver.rs"]
mod rs_pub_08_valid_semver;
#[path = "publish_integrity/rs_pub_09_publish_dry_run.rs"]
mod rs_pub_09_publish_dry_run;
#[path = "publish_integrity/rs_pub_10_no_path_deps_to_unpublishable.rs"]
mod rs_pub_10_no_path_deps_to_unpublishable;
#[path = "publish_integrity/rs_pub_11_interdependent_version_consistency.rs"]
mod rs_pub_11_interdependent_version_consistency;
#[path = "publish_integrity/rs_pub_12_crate_inventory.rs"]
mod rs_pub_12_crate_inventory;
#[path = "publish_integrity/rs_pub_13_docs_rs_metadata.rs"]
mod rs_pub_13_docs_rs_metadata;
#[path = "publish_integrity/rs_pub_14_include_exclude_inventory.rs"]
mod rs_pub_14_include_exclude_inventory;
#[path = "repo_policy/rs_release_01_license_file.rs"]
mod rs_release_01_license_file;
#[path = "repo_policy/rs_release_02_release_plz_exists.rs"]
mod rs_release_02_release_plz_exists;
#[path = "repo_policy/rs_release_03_release_plz_coverage.rs"]
mod rs_release_03_release_plz_coverage;
#[path = "repo_policy/rs_release_04_cliff_exists.rs"]
mod rs_release_04_cliff_exists;
#[path = "repo_policy/rs_release_05_release_plz_workflow.rs"]
mod rs_release_05_release_plz_workflow;
#[path = "repo_policy/rs_release_06_publish_dry_run_workflow.rs"]
mod rs_release_06_publish_dry_run_workflow;
#[path = "repo_policy/rs_release_07_registry_token.rs"]
mod rs_release_07_registry_token;
#[path = "repo_policy/rs_release_08_semver_checks_installed.rs"]
mod rs_release_08_semver_checks_installed;
#[path = "repo_inventory/rs_release_09_publish_status_inventory.rs"]
mod rs_release_09_publish_status_inventory;
#[path = "repo_inventory/rs_release_10_release_profile_inventory.rs"]
mod rs_release_10_release_profile_inventory;
#[path = "repo_inventory/rs_release_11_accidentally_publishable_internal_crates.rs"]
mod rs_release_11_accidentally_publishable_internal_crates;
#[path = "repo_inventory/rs_release_12_input_failures.rs"]
mod rs_release_12_input_failures;

use guardrail3_app_rs_family_mapper::{RsProjectSurface, RsReleaseRoute};
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
    surface: &RsProjectSurface,
    route: &RsReleaseRoute,
    tc: &dyn ToolChecker,
    thorough: bool,
) -> Vec<CheckResult> {
    let tree = surface.tree();
    let facts = collect(tree, route, tc, thorough);
    let mut results = Vec::new();

    for failure in &facts.input_failures {
        rs_release_12_input_failures::check(&ReleaseInputFailureInput::new(failure), &mut results);
    }

    for repo in &facts.repo {
        let input = RepoReleaseInput::new(repo);
        rs_release_01_license_file::check(&input, &mut results);
        rs_release_02_release_plz_exists::check(&input, &mut results);
        rs_release_03_release_plz_coverage::check(&input, &mut results);
        rs_release_04_cliff_exists::check(&input, &mut results);
        rs_release_05_release_plz_workflow::check(&input, &mut results);
        rs_release_06_publish_dry_run_workflow::check(&input, &mut results);
        rs_release_07_registry_token::check(&input, &mut results);
        rs_release_08_semver_checks_installed::check(&input, &mut results);
        rs_release_09_publish_status_inventory::check(&input, &mut results);
        rs_release_10_release_profile_inventory::check(&input, &mut results);
        rs_pub_12_crate_inventory::check(&input, &mut results);
    }

    for krate in &facts.crates {
        let input = PublishableCrateReleaseInput::new(krate);
        rs_pub_01_description_present::check(&input, &mut results);
        rs_pub_02_license_present::check(&input, &mut results);
        rs_pub_03_repository_present::check(&input, &mut results);
        rs_pub_04_readme_exists::check(&input, &mut results);
        rs_pub_05_readme_quality::check(&input, &mut results);
        rs_pub_06_keywords_present::check(&input, &mut results);
        rs_pub_07_categories_present::check(&input, &mut results);
        rs_pub_08_valid_semver::check(&input, &mut results);
        if thorough {
            rs_pub_09_publish_dry_run::check(&input, &mut results);
        }
        rs_pub_13_docs_rs_metadata::check(&input, &mut results);
        rs_pub_14_include_exclude_inventory::check(&input, &mut results);
        rs_bin_01_binary_release_workflow::check(&input, &facts.repo, &mut results);
        rs_bin_02_linux_target::check(&input, &facts.repo, &mut results);
        rs_bin_03_binstall_metadata::check(&input, &mut results);
        rs_release_11_accidentally_publishable_internal_crates::check(&input, &mut results);
    }

    for edge in &facts.edges {
        let input = ReleaseEdgeInput::new(edge);
        rs_pub_10_no_path_deps_to_unpublishable::check(&input, &mut results);
        rs_pub_11_interdependent_version_consistency::check(&input, &mut results);
    }

    results
}
