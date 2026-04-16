#[cfg(feature = "checks")]
use g3rs_arch_file_tree_checks_runtime as _;
use guardrail3_check_types as _;

#[cfg(feature = "checks")]
pub mod rs_arch_01_crate_has_facade;
#[cfg(feature = "checks")]
pub mod rs_arch_03_mod_rs_required;
#[cfg(feature = "checks")]
pub mod rs_arch_07a_structural_split;
