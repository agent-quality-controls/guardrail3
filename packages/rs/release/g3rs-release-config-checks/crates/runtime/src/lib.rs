#[cfg(test)]
use g3rs_release_config_checks_assertions as _;

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
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
