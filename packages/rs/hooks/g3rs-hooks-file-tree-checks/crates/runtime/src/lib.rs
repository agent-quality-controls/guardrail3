#[cfg(test)]
use g3rs_hooks_file_tree_checks_assertions as _;

mod hook_shared_01_pre_commit_exists;
mod hook_shared_02_hooks_path_configured;
mod hook_shared_03_modular_directory_inventory;
mod hook_shared_05_pre_commit_executable;
mod hook_shared_06_script_stats_inventory;
mod hook_shared_07_modular_scripts_inventory;
mod hook_shared_08_pre_commit_file_size_inventory;
mod hook_shared_09_local_override_inventory;
mod hook_shared_12_modular_scripts_executable;
mod hook_shared_17_execution_trust;
mod run;
#[cfg(test)]
mod test_support;

#[cfg(feature = "checks")]
pub use run::check;
