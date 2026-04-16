#[cfg(feature = "checks")]
use g3rs_toolchain_filetree_checks_runtime as _;
use guardrail3_check_types as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_toolchain_filetree_01_exists;
#[cfg(feature = "checks")]
pub mod rs_toolchain_filetree_04_legacy_file;
#[cfg(feature = "checks")]
pub mod run;
