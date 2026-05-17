//! Runtime rules for the `g3rs-deny-config-checks` family.

/// Rule implementation for `advisories`.
mod advisories;
/// Rule implementation for `allow override channel`.
mod allow_override_channel;
/// Rule implementation for `ban baseline complete`.
mod ban_baseline_complete;
/// Rule implementation for `bans`.
mod bans;
/// Baseline configuration helpers used by this family's rules.
mod baseline;
/// Rule implementation for `extra deny bans inventory`.
mod extra_deny_bans_inventory;
/// Rule implementation for `license exceptions inventory`.
mod license_exceptions_inventory;
/// Rule implementation for `licenses`.
mod licenses;
/// Family entry point that runs all rules.
mod run;
/// Rule implementation for `sources`.
mod sources;
/// Internal support helpers shared by this crate's rules.
mod support;
/// Rule implementation for `wrappers`.
mod wrappers;

#[cfg(feature = "checks")]
pub use run::check;
