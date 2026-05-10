use g3rs_garde_config_checks_runtime as _;

/// Shared assertion helpers and the `define_result_assertions!` macro.
mod common;

/// Assertions for the `additional_method_bans` rule.
#[cfg(feature = "checks")]
pub mod additional_method_bans;
/// Assertions for the `core_method_bans` rule.
#[cfg(feature = "checks")]
pub mod core_method_bans;
/// Assertions for the `dependency_present` rule.
#[cfg(feature = "checks")]
pub mod dependency_present;
/// Assertions for the `extractor_type_bans` rule.
#[cfg(feature = "checks")]
pub mod extractor_type_bans;
/// Assertions for the `reqwest_json_ban` rule.
#[cfg(feature = "checks")]
pub mod reqwest_json_ban;
/// Assertions for the family runner.
#[cfg(feature = "checks")]
pub mod run;
