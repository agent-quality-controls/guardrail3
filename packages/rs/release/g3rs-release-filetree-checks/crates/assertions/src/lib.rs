/// Shared assertion helpers and the `define_result_assertions!` macro.
mod common;

use g3rs_release_filetree_checks_runtime as _;

/// Assertions for the `cliff_exists` rule.
#[cfg(feature = "checks")]
pub mod cliff_exists;
/// Assertions for the `input_failures` rule.
#[cfg(feature = "checks")]
pub mod input_failures;
/// Assertions for the `license_file` rule.
#[cfg(feature = "checks")]
pub mod license_file;
/// Assertions for the `readme_exists` rule.
#[cfg(feature = "checks")]
pub mod readme_exists;
/// Assertions for the `release_plz_exists` rule.
#[cfg(feature = "checks")]
pub mod release_plz_exists;
/// Assertions for the family runner.
#[cfg(feature = "checks")]
pub mod run;
