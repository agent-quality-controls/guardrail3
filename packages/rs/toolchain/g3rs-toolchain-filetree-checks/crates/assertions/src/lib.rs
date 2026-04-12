use g3rs_toolchain_filetree_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_toolchain_filetree_01_exists;
#[cfg(feature = "checks")]
pub mod rs_toolchain_filetree_04_legacy_file;
