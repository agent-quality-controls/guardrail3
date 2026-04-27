use cargo_toml_parser as _;
use cliff_toml_parser as _;
use release_plz_toml_parser as _;
use semver as _;

#[cfg(test)]
use g3rs_release_config_checks_assertions as _;
mod accidentally_publishable;
mod binary_release_workflow;
mod binstall_metadata;
mod categories_present;
mod cliff_baseline;
mod crate_inventory;
mod description_present;
mod docs_rs_metadata;
mod include_exclude_inventory;
mod input_failures;
mod interdependent_version_consistency;
mod keywords_present;
#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod lib_tests;
mod license_present;
mod linux_release_target;
mod no_path_deps_to_unpublishable;
mod publish_dry_run;
mod publish_must_be_explicit;
mod publish_status_inventory;
mod release_plz_baseline;
mod release_profile_inventory;
mod repository_present;
mod run;
mod semver_checks_installed;
mod support;
mod valid_semver;

#[cfg(feature = "checks")]
pub use run::check;
