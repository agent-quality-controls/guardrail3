#[cfg(test)]
use g3rs_hooks_file_tree_checks_assertions as _;

mod execution_trust;
mod hooks_path_configured;
mod local_override_inventory;
mod modular_directory_inventory;
mod modular_scripts_executable;
mod modular_scripts_inventory;
mod pre_commit_executable;
mod pre_commit_exists;
mod pre_commit_file_size_inventory;
mod run;
mod script_stats_inventory;

#[cfg(feature = "checks")]
pub use run::check;
