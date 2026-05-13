//! Runtime rules for the `g3rs-hooks-config-checks` family.

#[cfg(test)]
use g3rs_hooks_config_checks_assertions as _;

/// Rule implementation for `cargo dupes installed`.
mod cargo_dupes_installed;
/// Rule implementation for `contract required tools installed`.
mod contract_required_tools_installed;
/// Rule implementation for `guardrail binary available`.
mod guardrail_binary_available;
/// Rule implementation for loaded family hook contract inventory.
mod hook_contract_inventory;
/// Rule implementation for `required tools installed`.
mod required_tools_installed;
/// Family entry point that runs all rules.
mod run;
/// Internal support helpers shared by this crate's rules.
mod support;

#[cfg(feature = "checks")]
pub use run::check;
