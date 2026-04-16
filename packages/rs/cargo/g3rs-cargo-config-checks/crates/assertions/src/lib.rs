#[cfg(feature = "checks")]
use g3rs_cargo_config_checks_runtime as _;
use guardrail3_check_types as _;

/// Shared low-level result matching used by the rule-specific proof modules.
mod common;

#[cfg(feature = "checks")]
pub mod rs_cargo_config_01_workspace_lints;
#[cfg(feature = "checks")]
pub mod rs_cargo_config_02_lint_levels;
#[cfg(feature = "checks")]
pub mod rs_cargo_config_03_workspace_metadata;
#[cfg(feature = "checks")]
pub mod rs_cargo_config_04_priority_order;
#[cfg(feature = "checks")]
pub mod rs_cargo_config_05_resolver;
#[cfg(feature = "checks")]
pub mod rs_cargo_config_06_disallowed_macros_deny;
#[cfg(feature = "checks")]
pub mod rs_cargo_config_07_approved_allow_inventory;
#[cfg(feature = "checks")]
pub mod rs_cargo_config_08_workspace_lints_inherited;
#[cfg(feature = "checks")]
pub mod rs_cargo_config_09_no_weakened_overrides;
#[cfg(feature = "checks")]
pub mod rs_cargo_config_10_member_edition_drift;
#[cfg(feature = "checks")]
pub mod rs_cargo_config_11_unapproved_allow_entries;
#[cfg(feature = "checks")]
pub mod rs_cargo_config_12_member_local_allows_forbidden;
#[cfg(feature = "checks")]
pub mod rs_cargo_config_13_rust_version_policy;
