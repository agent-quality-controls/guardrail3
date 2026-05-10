#[cfg(test)]
use g3rs_hooks_file_tree_checks_assertions as _;

/// `execution_trust` module.
mod execution_trust;
/// `hooks_path_configured` module.
mod hooks_path_configured;
/// `local_override_inventory` module.
mod local_override_inventory;
/// `modular_directory_inventory` module.
mod modular_directory_inventory;
/// `modular_scripts_executable` module.
mod modular_scripts_executable;
/// `modular_scripts_inventory` module.
mod modular_scripts_inventory;
/// `pre_commit_executable` module.
mod pre_commit_executable;
/// `pre_commit_exists` module.
mod pre_commit_exists;
/// `pre_commit_file_size_inventory` module.
mod pre_commit_file_size_inventory;
/// `run` module.
mod run;
/// `script_stats_inventory` module.
mod script_stats_inventory;

#[cfg(feature = "checks")]
pub use run::check;
