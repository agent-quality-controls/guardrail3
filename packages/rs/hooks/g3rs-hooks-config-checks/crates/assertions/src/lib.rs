use g3rs_hooks_config_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod cargo_dupes_installed;
#[cfg(feature = "checks")]
pub mod guardrail_binary_available;
#[cfg(feature = "checks")]
pub mod required_tools_installed;
