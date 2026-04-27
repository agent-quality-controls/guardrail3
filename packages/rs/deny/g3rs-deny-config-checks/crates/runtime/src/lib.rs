mod advisories;
mod allow_override_channel;
mod ban_baseline_complete;
mod bans;
mod baseline;
mod extra_deny_bans_inventory;
mod license_exceptions_inventory;
mod licenses;
mod run;
mod sources;
mod support;
mod wrappers;

#[cfg(test)]
use g3rs_deny_config_checks_assertions as _;
#[cfg(test)]
use test_support as _;

#[cfg(feature = "checks")]
pub use run::check;
