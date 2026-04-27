use g3rs_fmt_filetree_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod dual_file_conflict;
#[cfg(feature = "checks")]
pub mod exists;
#[cfg(feature = "checks")]
pub mod per_crate_override;
#[cfg(feature = "checks")]
pub mod run;
