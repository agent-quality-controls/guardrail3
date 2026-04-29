mod declared_entrypoint_exists;
mod run;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_arch_file_tree_checks_assertions as _;
