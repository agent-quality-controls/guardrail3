use g3rs_garde_config_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod additional_method_bans;
#[cfg(feature = "checks")]
pub mod core_method_bans;
#[cfg(feature = "checks")]
pub mod dependency_present;
#[cfg(feature = "checks")]
pub mod extractor_type_bans;
#[cfg(feature = "checks")]
pub mod reqwest_json_ban;
#[cfg(feature = "checks")]
pub mod run;
