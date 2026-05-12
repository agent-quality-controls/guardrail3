use g3rs_deny_config_checks_runtime as _;

/// Shared helpers used by per-rule assertion modules in this crate.
mod common;

#[cfg(feature = "checks")]
pub mod advisories;
#[cfg(feature = "checks")]
pub mod bans;
#[cfg(feature = "checks")]
pub mod licenses;
#[cfg(feature = "checks")]
pub mod sources;

#[cfg(feature = "checks")]
pub mod allow_override_channel;
#[cfg(feature = "checks")]
pub mod ban_baseline_complete;
#[cfg(feature = "checks")]
pub mod extra_deny_bans_inventory;
#[cfg(feature = "checks")]
pub mod wrappers;

#[cfg(feature = "checks")]
pub mod license_exceptions_inventory;
