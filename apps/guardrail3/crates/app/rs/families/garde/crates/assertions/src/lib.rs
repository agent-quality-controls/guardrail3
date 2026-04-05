use guardrail3_app_rs_family_garde as _;

mod common;
#[cfg(feature = "checks")]
pub mod facts;

#[cfg(feature = "checks")]
pub mod rs_garde_01_dependency_present;
#[cfg(feature = "checks")]
pub mod rs_garde_02_core_method_bans;
#[cfg(feature = "checks")]
pub mod rs_garde_03_extractor_type_bans;
#[cfg(feature = "checks")]
pub mod rs_garde_04_reqwest_json_ban;
#[cfg(feature = "checks")]
pub mod rs_garde_06_additional_method_bans;
#[cfg(feature = "checks")]
pub mod rs_garde_10_input_failures;
