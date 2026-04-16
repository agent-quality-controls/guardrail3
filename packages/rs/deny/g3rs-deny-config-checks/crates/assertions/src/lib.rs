use g3rs_deny_config_checks_runtime as _;

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
pub mod rs_deny_config_23_ban_baseline_complete;
#[cfg(feature = "checks")]
pub mod rs_deny_config_25_allow_override_channel;
#[cfg(feature = "checks")]
pub mod rs_deny_config_26_extra_deny_bans_inventory;
#[cfg(feature = "checks")]
pub mod rs_deny_config_27_wrappers;

#[cfg(feature = "checks")]
pub mod rs_deny_config_24_license_exceptions_inventory;
