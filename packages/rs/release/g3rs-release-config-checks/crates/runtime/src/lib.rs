use cargo_toml_parser as _;
use cliff_toml_parser as _;
use release_plz_toml_parser as _;
use semver as _;

#[cfg(test)]
use g3rs_release_config_checks_assertions as _;

mod rs_release_config_00_publish_must_be_explicit;
mod rs_release_config_01_description_present;
mod rs_release_config_02_license_present;
mod rs_release_config_03_repository_present;
mod rs_release_config_04_keywords_present;
mod rs_release_config_05_categories_present;
mod rs_release_config_06_valid_semver;
mod rs_release_config_07_docs_rs_metadata;
mod rs_release_config_08_binstall_metadata;
mod rs_release_config_09_accidentally_publishable;
mod rs_release_config_10_release_plz_baseline;
mod rs_release_config_11_cliff_baseline;
mod rs_release_config_15_semver_checks_installed;
mod rs_release_config_16_publish_status_inventory;
mod rs_release_config_17_release_profile_inventory;
mod rs_release_config_18_publish_dry_run;
mod rs_release_config_19_no_path_deps_to_unpublishable;
mod rs_release_config_20_interdependent_version_consistency;
mod rs_release_config_21_crate_inventory;
mod rs_release_config_22_include_exclude_inventory;
mod rs_release_config_23_binary_release_workflow;
mod rs_release_config_24_linux_release_target;
mod rs_release_config_25_input_failures;
mod run;
mod support;
#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod lib_tests;

#[cfg(feature = "checks")]
pub use run::check;
