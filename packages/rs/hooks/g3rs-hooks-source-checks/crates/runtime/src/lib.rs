mod bootstrap;
#[cfg(test)]
#[allow(unused_imports)]
mod cargo_deny_step_present;
#[allow(dead_code, unused_imports)]
mod cargo_dupes_excludes;
#[cfg(test)]
#[allow(unused_imports)]
mod cargo_dupes_step_present;
#[cfg(test)]
#[allow(unused_imports)]
mod cargo_machete_step_present;
#[allow(dead_code, unused_imports)]
mod clippy_denies_warnings;
#[cfg(test)]
#[allow(unused_imports)]
mod clippy_step_present;
mod compat;
#[cfg(test)]
#[allow(unused_imports)]
mod config_changes_trigger_validation;
mod contract_critical_command_not_fail_open;
mod contract_trigger_coverage;
#[cfg(test)]
#[allow(unused_imports)]
mod duplication_tool_is_cargo_dupes;
mod facts;
#[cfg(test)]
#[allow(unused_imports)]
mod fmt_step_present;
mod gitleaks_step_present;
#[cfg(test)]
#[allow(unused_imports)]
mod guardrail_validate_staged_present;
mod inputs;
mod required_contract_command_present;
mod run;
#[cfg(test)]
#[allow(unused_imports)]
mod shared_target_dir_present;
mod shell_safety;
mod support;
#[cfg(test)]
#[allow(unused_imports)]
mod test_step_present;
#[cfg(test)]
#[allow(unused_imports)]
mod test_uses_workspace;
mod workflow;

#[cfg(feature = "checks")]
pub use run::{check, check_all};
