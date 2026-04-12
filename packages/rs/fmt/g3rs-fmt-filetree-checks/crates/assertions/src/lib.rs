use g3rs_fmt_filetree_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_fmt_filetree_01_exists;
#[cfg(feature = "checks")]
pub mod rs_fmt_filetree_05_per_crate_override;
#[cfg(feature = "checks")]
pub mod rs_fmt_filetree_08_dual_file_conflict;
