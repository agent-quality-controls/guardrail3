use g3rs_hooks_source_checks_runtime as _;

#[cfg(feature = "checks")]
pub mod bootstrap;
mod common;
#[cfg(feature = "checks")]
pub mod shell_safety;
#[cfg(feature = "checks")]
pub mod workflow;

#[cfg(feature = "checks")]
pub mod hook_rs_01_fmt_step_present;
#[cfg(feature = "checks")]
pub mod hook_rs_02_clippy_step_present;
#[cfg(feature = "checks")]
pub mod hook_rs_03_cargo_deny_step_present;
#[cfg(feature = "checks")]
pub mod hook_rs_04_test_step_present;
#[cfg(feature = "checks")]
pub mod hook_rs_05_cargo_machete_step_present;
#[cfg(feature = "checks")]
pub mod hook_rs_07_duplication_tool_is_cargo_dupes;
#[cfg(feature = "checks")]
pub mod hook_rs_08_guardrail_validate_staged_present;
#[cfg(feature = "checks")]
pub mod hook_rs_09_clippy_denies_warnings;
#[cfg(feature = "checks")]
pub mod hook_rs_10_test_uses_workspace;
#[cfg(feature = "checks")]
pub mod hook_rs_11_gitleaks_step_present;
#[cfg(feature = "checks")]
pub mod hook_rs_12_cargo_dupes_step_present;
#[cfg(feature = "checks")]
pub mod hook_rs_13_cargo_dupes_excludes;
#[cfg(feature = "checks")]
pub mod hook_rs_16_config_changes_trigger_validation;
