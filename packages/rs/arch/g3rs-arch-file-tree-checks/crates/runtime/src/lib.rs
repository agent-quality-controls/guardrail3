#[cfg(test)]
use g3rs_arch_file_tree_checks_assertions as _;

mod rs_arch_01_crate_has_facade;
mod rs_arch_03_mod_rs_required;
mod rs_arch_07a_structural_split;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
