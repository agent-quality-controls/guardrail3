/// Rule: each declared entrypoint file must exist on disk.
mod declared_entrypoint_exists;
/// Family runner that dispatches to per-rule check modules.
mod run;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_arch_file_tree_checks_assertions as _;
