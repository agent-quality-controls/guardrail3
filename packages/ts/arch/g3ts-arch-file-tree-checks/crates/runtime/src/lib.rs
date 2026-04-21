mod run;
mod ts_arch_filetree_01_declared_entrypoint_exists;
mod ts_arch_filetree_02_structural_split;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_arch_file_tree_checks_assertions as _;
