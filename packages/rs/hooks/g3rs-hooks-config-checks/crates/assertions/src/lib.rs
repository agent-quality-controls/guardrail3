use g3rs_hooks_config_checks_runtime as _;

/// Shared helpers used by per-rule assertion modules in this crate.
mod common;

#[cfg(feature = "checks")]
pub mod cargo_dupes_installed;
#[cfg(feature = "checks")]
pub mod contract_required_tools_installed;
#[cfg(feature = "checks")]
pub mod guardrail_binary_available;
#[cfg(feature = "checks")]
pub mod required_tools_installed;
