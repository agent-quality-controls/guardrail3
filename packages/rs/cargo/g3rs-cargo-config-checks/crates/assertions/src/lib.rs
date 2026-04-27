#[cfg(feature = "checks")]
use g3rs_cargo_config_checks_runtime as _;
use guardrail3_check_types as _;

/// Shared low-level result matching used by the rule-specific proof modules.
mod common;

#[cfg(feature = "checks")]
pub mod approved_allow_inventory;
#[cfg(feature = "checks")]
pub mod disallowed_macros_deny;
#[cfg(feature = "checks")]
pub mod lint_levels;
#[cfg(feature = "checks")]
pub mod member_edition_drift;
#[cfg(feature = "checks")]
pub mod member_local_allows_forbidden;
#[cfg(feature = "checks")]
pub mod no_weakened_overrides;
#[cfg(feature = "checks")]
pub mod priority_order;
#[cfg(feature = "checks")]
pub mod resolver;
#[cfg(feature = "checks")]
pub mod rust_version_policy;
#[cfg(feature = "checks")]
pub mod unapproved_allow_entries;
#[cfg(feature = "checks")]
pub mod workspace_lints;
#[cfg(feature = "checks")]
pub mod workspace_lints_inherited;
#[cfg(feature = "checks")]
pub mod workspace_metadata;
