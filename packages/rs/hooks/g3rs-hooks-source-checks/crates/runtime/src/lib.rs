mod bootstrap;
mod cargo_deny_step_present;
mod cargo_dupes_excludes;
mod cargo_dupes_step_present;
mod cargo_machete_step_present;
mod clippy_denies_warnings;
mod clippy_step_present;
mod compat;
mod config_changes_trigger_validation;
mod duplication_tool_is_cargo_dupes;
mod facts;
mod fmt_step_present;
mod gitleaks_step_present;
mod guardrail_validate_staged_present;
mod inputs;
mod run;
mod shared_target_dir_present;
mod shell_safety;
mod support;
mod test_step_present;
mod test_uses_workspace;
mod workflow;

#[cfg(feature = "checks")]
pub use run::check;
