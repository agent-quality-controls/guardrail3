mod advisories;
mod bans;
mod baseline;
mod licenses;
mod rs_deny_config_23_ban_baseline_complete;
mod rs_deny_config_24_license_exceptions_inventory;
mod rs_deny_config_25_allow_override_channel;
mod rs_deny_config_26_extra_deny_bans_inventory;
mod rs_deny_config_27_wrappers;
mod run;
mod sources;
mod support;

#[cfg(test)]
use g3rs_deny_config_checks_assertions as _;
#[cfg(test)]
use test_support as _;

#[cfg(feature = "checks")]
pub use run::check;
