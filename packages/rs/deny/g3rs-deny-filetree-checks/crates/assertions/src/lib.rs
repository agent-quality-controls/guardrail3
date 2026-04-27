use g3rs_deny_filetree_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod coverage;
#[cfg(feature = "checks")]
pub mod run;
#[cfg(feature = "checks")]
pub mod shadowing;
