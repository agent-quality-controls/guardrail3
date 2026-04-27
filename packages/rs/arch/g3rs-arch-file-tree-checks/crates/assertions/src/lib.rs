#[cfg(feature = "checks")]
use g3rs_arch_file_tree_checks_runtime as _;
use guardrail3_check_types as _;

#[cfg(feature = "checks")]
pub mod crate_has_facade;
#[cfg(feature = "checks")]
pub mod mod_rs_required;
#[cfg(feature = "checks")]
pub mod structural_split;
