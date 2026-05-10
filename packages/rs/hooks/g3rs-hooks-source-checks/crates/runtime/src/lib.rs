/// `bootstrap` module.
mod bootstrap;
/// `cargo_dupes_excludes` module.
#[allow(dead_code, unused_imports)] // reason: legacy cargo dupes-excludes rule retained behind feature flag.
#[rustfmt::skip]
mod cargo_dupes_excludes;
/// `clippy_denies_warnings` module.
#[allow(dead_code, unused_imports)] // reason: legacy clippy-denies-warnings rule retained behind feature flag.
#[rustfmt::skip]
mod clippy_denies_warnings;
/// `compat` module.
mod compat;
/// `contract_critical_command_not_fail_open` module.
mod contract_critical_command_not_fail_open;
/// `contract_trigger_coverage` module.
mod contract_trigger_coverage;
/// `facts` module.
mod facts;
/// `gitleaks_step_present` module.
mod gitleaks_step_present;
/// `inputs` module.
mod inputs;
/// `required_contract_command_present` module.
mod required_contract_command_present;
/// `routing` module.
mod routing;
/// `run` module.
mod run;
/// `shell_safety` module.
mod shell_safety;
/// `support` module.
mod support;
/// `workflow` module.
mod workflow;

#[cfg(feature = "checks")]
pub use run::{check, check_all};
