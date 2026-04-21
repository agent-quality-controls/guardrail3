mod bootstrap;
mod compat;
mod facts;
mod hook_rs_01_fmt_step_present;
mod hook_rs_02_clippy_step_present;
mod hook_rs_03_cargo_deny_step_present;
mod hook_rs_04_test_step_present;
mod hook_rs_05_cargo_machete_step_present;
mod hook_rs_07_duplication_tool_is_cargo_dupes;
mod hook_rs_08_guardrail_validate_staged_present;
mod hook_rs_09_clippy_denies_warnings;
mod hook_rs_10_test_uses_workspace;
mod hook_rs_11_gitleaks_step_present;
mod hook_rs_12_cargo_dupes_step_present;
mod hook_rs_13_cargo_dupes_excludes;
mod hook_rs_16_config_changes_trigger_validation;
mod hook_rs_17_shared_target_dir_present;
mod inputs;
mod run;
mod shell_safety;
mod support;
mod workflow;

#[cfg(feature = "checks")]
pub use run::check;
