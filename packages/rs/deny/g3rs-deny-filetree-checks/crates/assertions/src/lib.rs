use g3rs_deny_filetree_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_deny_filetree_01_coverage;
#[cfg(feature = "checks")]
pub mod rs_deny_filetree_03_shadowing;
