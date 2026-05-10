use g3rs_hooks_file_tree_checks_runtime as _;

/// `common` module.
mod common;

#[cfg(feature = "checks")]
pub mod execution_trust;
#[cfg(feature = "checks")]
pub mod hooks_path_configured;
#[cfg(feature = "checks")]
pub mod local_override_inventory;
#[cfg(feature = "checks")]
pub mod modular_directory_inventory;
#[cfg(feature = "checks")]
pub mod modular_scripts_executable;
#[cfg(feature = "checks")]
pub mod modular_scripts_inventory;
#[cfg(feature = "checks")]
pub mod pre_commit_executable;
#[cfg(feature = "checks")]
pub mod pre_commit_exists;
#[cfg(feature = "checks")]
pub mod pre_commit_file_size_inventory;
#[cfg(feature = "checks")]
pub mod script_stats_inventory;
