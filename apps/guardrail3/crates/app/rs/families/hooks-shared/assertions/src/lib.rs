use guardrail3_app_rs_family_hooks_shared as _;

mod common;

#[path = "bootstrap/hook_shared_01_pre_commit_exists.rs"]
pub mod hook_shared_01_pre_commit_exists;
#[path = "bootstrap/hook_shared_02_hooks_path_configured.rs"]
pub mod hook_shared_02_hooks_path_configured;
#[path = "bootstrap/hook_shared_03_modular_directory_inventory.rs"]
pub mod hook_shared_03_modular_directory_inventory;
#[path = "bootstrap/hook_shared_04_dispatcher_pattern.rs"]
pub mod hook_shared_04_dispatcher_pattern;
#[path = "bootstrap/hook_shared_05_pre_commit_executable.rs"]
pub mod hook_shared_05_pre_commit_executable;
#[path = "bootstrap/hook_shared_06_script_stats_inventory.rs"]
pub mod hook_shared_06_script_stats_inventory;
#[path = "inventories/hook_shared_07_modular_scripts_inventory.rs"]
pub mod hook_shared_07_modular_scripts_inventory;
#[path = "inventories/hook_shared_08_pre_commit_file_size_inventory.rs"]
pub mod hook_shared_08_pre_commit_file_size_inventory;
#[path = "inventories/hook_shared_09_local_override_inventory.rs"]
pub mod hook_shared_09_local_override_inventory;
#[path = "shell_safety/hook_shared_10_shell_error_handling.rs"]
pub mod hook_shared_10_shell_error_handling;
#[path = "shell_safety/hook_shared_11_valid_shebang.rs"]
pub mod hook_shared_11_valid_shebang;
#[path = "inventories/hook_shared_12_modular_scripts_executable.rs"]
pub mod hook_shared_12_modular_scripts_executable;
#[path = "shell_safety/hook_shared_13_no_unconditional_exit_zero.rs"]
pub mod hook_shared_13_no_unconditional_exit_zero;
#[path = "shell_safety/hook_shared_14_no_bypass_instructions.rs"]
pub mod hook_shared_14_no_bypass_instructions;
#[path = "workflow/hook_shared_15_merge_conflict_step_present.rs"]
pub mod hook_shared_15_merge_conflict_step_present;
#[path = "workflow/hook_shared_16_file_size_step_present.rs"]
pub mod hook_shared_16_file_size_step_present;
#[path = "inventories/hook_shared_17_execution_trust.rs"]
pub mod hook_shared_17_execution_trust;
#[path = "shell_safety/hook_shared_18_executable_command_context_only.rs"]
pub mod hook_shared_18_executable_command_context_only;
#[path = "shell_safety/hook_shared_19_real_dispatcher_syntax_only.rs"]
pub mod hook_shared_19_real_dispatcher_syntax_only;
#[path = "shell_safety/hook_shared_20_concrete_lockfile_command.rs"]
pub mod hook_shared_20_concrete_lockfile_command;
#[path = "shell_safety/hook_shared_21_no_fail_open_wrappers.rs"]
pub mod hook_shared_21_no_fail_open_wrappers;
pub mod hook_shell;
