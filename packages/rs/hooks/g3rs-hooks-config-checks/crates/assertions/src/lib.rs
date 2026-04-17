use g3rs_hooks_config_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod hook_rs_06_required_tools_installed;
#[cfg(feature = "checks")]
pub mod hook_rs_14_guardrail_binary_available;
#[cfg(feature = "checks")]
pub mod hook_rs_15_cargo_dupes_installed;
