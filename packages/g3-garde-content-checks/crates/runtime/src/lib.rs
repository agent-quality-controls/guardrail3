mod rs_garde_01_dependency_present;
mod rs_garde_02_core_method_bans;
mod rs_garde_03_extractor_type_bans;
mod rs_garde_04_reqwest_json_ban;
mod rs_garde_06_additional_method_bans;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::{check_clippy_bans, check_dependency_present};
