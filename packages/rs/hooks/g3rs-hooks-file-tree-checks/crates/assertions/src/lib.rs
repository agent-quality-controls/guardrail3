#![allow(dead_code, unreachable_pub, unused_imports)]

mod common;

#[cfg(feature = "assertions")]
pub mod hook_shared_01_pre_commit_exists;
#[cfg(feature = "assertions")]
pub mod hook_shared_02_hooks_path_configured;
#[cfg(feature = "assertions")]
pub mod hook_shared_03_modular_directory_inventory;
#[cfg(feature = "assertions")]
pub mod hook_shared_05_pre_commit_executable;
#[cfg(feature = "assertions")]
pub mod hook_shared_06_script_stats_inventory;
#[cfg(feature = "assertions")]
pub mod hook_shared_07_modular_scripts_inventory;
#[cfg(feature = "assertions")]
pub mod hook_shared_08_pre_commit_file_size_inventory;
#[cfg(feature = "assertions")]
pub mod hook_shared_09_local_override_inventory;
#[cfg(feature = "assertions")]
pub mod hook_shared_12_modular_scripts_executable;
#[cfg(feature = "assertions")]
pub mod hook_shared_17_execution_trust;
