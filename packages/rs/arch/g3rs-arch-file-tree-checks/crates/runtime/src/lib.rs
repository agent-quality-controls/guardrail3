#[cfg(test)]
use g3rs_arch_file_tree_checks_assertions as _;

mod crate_has_facade;
mod mod_rs_required;
mod run;
mod structural_split;

#[cfg(feature = "checks")]
pub use run::check;
