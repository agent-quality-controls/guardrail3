#[cfg(feature = "checks")]
use g3rs_toolchain_filetree_checks_runtime as _;
use guardrail3_check_types as _;

mod common;

#[cfg(feature = "checks")]
pub mod exists;
#[cfg(feature = "checks")]
pub mod legacy_file;
#[cfg(feature = "checks")]
pub mod run;
