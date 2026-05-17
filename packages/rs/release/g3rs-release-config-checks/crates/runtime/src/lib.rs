use cargo_toml_parser as _;
use cliff_toml_parser as _;
use release_plz_toml_parser as _;
use semver as _;

/// `accidentally_publishable` module.
mod accidentally_publishable;
/// `binary_release_workflow` module.
mod binary_release_workflow;
/// `binstall_metadata` module.
mod binstall_metadata;
/// `categories_present` module.
mod categories_present;
/// `cliff_baseline` module.
mod cliff_baseline;
/// `crate_inventory` module.
mod crate_inventory;
/// `description_present` module.
mod description_present;
/// `docs_rs_metadata` module.
mod docs_rs_metadata;
/// `include_exclude_inventory` module.
mod include_exclude_inventory;
/// `input_failures` module.
mod input_failures;
/// `interdependent_version_consistency` module.
mod interdependent_version_consistency;
/// `keywords_present` module.
mod keywords_present;
/// `license_present` module.
mod license_present;
/// `linux_release_target` module.
mod linux_release_target;
/// `no_path_deps_to_unpublishable` module.
mod no_path_deps_to_unpublishable;
/// `publish_dry_run` module.
mod publish_dry_run;
/// `publish_must_be_explicit` module.
mod publish_must_be_explicit;
/// `publish_status_inventory` module.
mod publish_status_inventory;
/// `release_plz_baseline` module.
mod release_plz_baseline;
/// `release_profile_inventory` module.
mod release_profile_inventory;
/// `repository_present` module.
mod repository_present;
/// `run` module.
mod run;
/// `semver_checks_installed` module.
mod semver_checks_installed;
/// `support` module.
mod support;
/// `valid_semver` module.
mod valid_semver;

#[cfg(feature = "checks")]
pub use run::check;
