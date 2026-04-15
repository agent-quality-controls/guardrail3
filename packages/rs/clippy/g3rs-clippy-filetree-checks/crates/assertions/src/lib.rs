use g3rs_clippy_filetree_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_clippy_filetree_01_coverage_exists;
#[cfg(feature = "checks")]
pub mod rs_clippy_filetree_02_same_root_conflict;
#[cfg(feature = "checks")]
pub mod run;
