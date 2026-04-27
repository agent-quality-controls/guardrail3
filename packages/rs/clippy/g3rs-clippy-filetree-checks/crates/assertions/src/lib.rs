use g3rs_clippy_filetree_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod coverage_exists;
#[cfg(feature = "checks")]
pub mod run;
#[cfg(feature = "checks")]
pub mod same_root_conflict;
