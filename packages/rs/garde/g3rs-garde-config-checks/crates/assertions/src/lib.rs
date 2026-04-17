use g3rs_garde_config_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_garde_config_01_dependency_present;
#[cfg(feature = "checks")]
pub mod rs_garde_config_02_core_method_bans;
#[cfg(feature = "checks")]
pub mod rs_garde_config_03_extractor_type_bans;
#[cfg(feature = "checks")]
pub mod rs_garde_config_04_reqwest_json_ban;
#[cfg(feature = "checks")]
pub mod rs_garde_config_05_additional_method_bans;
#[cfg(feature = "checks")]
pub mod run;
