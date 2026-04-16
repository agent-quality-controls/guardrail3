#[cfg(feature = "checks")]
use g3rs_deps_filetree_checks_runtime as _;
use guardrail3_check_types as _;

#[cfg(feature = "checks")]
pub mod rs_deps_filetree_09_cargo_lock_present;
#[cfg(feature = "checks")]
pub mod rs_deps_filetree_10_gitignore_not_ignoring_cargo_lock;
#[cfg(feature = "checks")]
pub mod run;
