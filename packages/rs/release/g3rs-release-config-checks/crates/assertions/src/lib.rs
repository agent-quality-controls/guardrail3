use g3rs_release_config_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_release_config_00_publish_must_be_explicit;
#[cfg(feature = "checks")]
pub mod rs_release_config_01_description_present;
#[cfg(feature = "checks")]
pub mod rs_release_config_02_license_present;
#[cfg(feature = "checks")]
pub mod rs_release_config_03_repository_present;
#[cfg(feature = "checks")]
pub mod rs_release_config_04_keywords_present;
#[cfg(feature = "checks")]
pub mod rs_release_config_05_categories_present;
#[cfg(feature = "checks")]
pub mod rs_release_config_06_valid_semver;
#[cfg(feature = "checks")]
pub mod rs_release_config_07_docs_rs_metadata;
#[cfg(feature = "checks")]
pub mod rs_release_config_08_binstall_metadata;
#[cfg(feature = "checks")]
pub mod rs_release_config_09_accidentally_publishable;
#[cfg(feature = "checks")]
pub mod rs_release_config_10_release_plz_baseline;
#[cfg(feature = "checks")]
pub mod rs_release_config_11_cliff_baseline;
#[cfg(feature = "checks")]
pub mod rs_release_config_19_no_path_deps_to_unpublishable;
#[cfg(feature = "checks")]
pub mod rs_release_config_20_interdependent_version_consistency;
#[cfg(feature = "checks")]
pub mod run;
