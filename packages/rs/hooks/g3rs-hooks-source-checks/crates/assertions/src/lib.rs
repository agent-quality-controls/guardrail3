use g3rs_hooks_source_checks_runtime as _;

#[cfg(feature = "checks")]
pub mod bootstrap;
mod common;
#[cfg(feature = "checks")]
pub mod shell_safety;
#[cfg(feature = "checks")]
pub mod workflow;

#[cfg(feature = "checks")]
pub mod cargo_deny_step_present;
#[cfg(feature = "checks")]
pub mod cargo_dupes_excludes;
#[cfg(feature = "checks")]
pub mod cargo_dupes_step_present;
#[cfg(feature = "checks")]
pub mod cargo_machete_step_present;
#[cfg(feature = "checks")]
pub mod clippy_denies_warnings;
#[cfg(feature = "checks")]
pub mod clippy_step_present;
#[cfg(feature = "checks")]
pub mod config_changes_trigger_validation;
#[cfg(feature = "checks")]
pub mod duplication_tool_is_cargo_dupes;
#[cfg(feature = "checks")]
pub mod fmt_step_present;
#[cfg(feature = "checks")]
pub mod gitleaks_step_present;
#[cfg(feature = "checks")]
pub mod guardrail_validate_staged_present;
#[cfg(feature = "checks")]
pub mod shared_target_dir_present;
#[cfg(feature = "checks")]
pub mod test_step_present;
#[cfg(feature = "checks")]
pub mod test_uses_workspace;
