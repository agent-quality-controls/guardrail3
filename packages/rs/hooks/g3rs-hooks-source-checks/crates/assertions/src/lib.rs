use g3rs_hooks_source_checks_runtime as _;

#[cfg(feature = "checks")]
pub mod bootstrap;
/// `common` module.
mod common;
#[cfg(feature = "checks")]
pub mod shell_safety;
#[cfg(feature = "checks")]
pub mod workflow;

#[cfg(feature = "checks")]
pub mod cargo_dupes_excludes;
#[cfg(feature = "checks")]
pub mod clippy_denies_warnings;
#[cfg(feature = "checks")]
pub mod contract_critical_command_not_fail_open;
#[cfg(feature = "checks")]
pub mod contract_trigger_coverage;
#[cfg(feature = "checks")]
pub mod dispatch;
#[cfg(feature = "checks")]
pub mod gitleaks_step_present;
#[cfg(feature = "checks")]
pub mod required_contract_command_present;
#[cfg(feature = "checks")]
pub mod routing;
#[cfg(feature = "checks")]
pub mod run;
