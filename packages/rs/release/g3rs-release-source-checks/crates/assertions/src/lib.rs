use g3rs_release_source_checks_runtime as _;

/// Shared assertion helpers and macros.
mod common;

#[cfg(feature = "checks")]
pub mod input_failures;
#[cfg(feature = "checks")]
pub mod readme_quality;
#[cfg(feature = "checks")]
pub mod run;
