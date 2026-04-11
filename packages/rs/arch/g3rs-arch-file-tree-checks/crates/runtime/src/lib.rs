#[cfg(test)]
use g3rs_arch_file_tree_checks_assertions as _;

mod rs_arch_01_crate_has_facade;
mod rs_arch_03_mod_rs_required;
mod rs_arch_07_force_crate_split;
mod run;
#[cfg(test)]
mod test_support;

#[cfg(feature = "checks")]
pub use run::check;
