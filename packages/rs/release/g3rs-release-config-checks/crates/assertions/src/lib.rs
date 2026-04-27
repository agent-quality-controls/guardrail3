use g3rs_release_config_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod accidentally_publishable;
#[cfg(feature = "checks")]
pub mod binstall_metadata;
#[cfg(feature = "checks")]
pub mod categories_present;
#[cfg(feature = "checks")]
pub mod cliff_baseline;
#[cfg(feature = "checks")]
pub mod description_present;
#[cfg(feature = "checks")]
pub mod docs_rs_metadata;
#[cfg(feature = "checks")]
pub mod interdependent_version_consistency;
#[cfg(feature = "checks")]
pub mod keywords_present;
#[cfg(feature = "checks")]
pub mod license_present;
#[cfg(feature = "checks")]
pub mod no_path_deps_to_unpublishable;
#[cfg(feature = "checks")]
pub mod publish_must_be_explicit;
#[cfg(feature = "checks")]
pub mod release_plz_baseline;
#[cfg(feature = "checks")]
pub mod repository_present;
#[cfg(feature = "checks")]
pub mod run;
#[cfg(feature = "checks")]
pub mod valid_semver;
