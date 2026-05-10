/// Checks for duplicate setting keys in the root `.npmrc`.
mod duplicate_keys;
/// Inventories extra settings that fall outside the required baseline.
mod extra_settings_inventory;
/// Checks that all required settings are declared in the root `.npmrc`.
mod required_settings_present;
/// Checks that required settings declare values strong enough to enforce
/// the policy.
mod required_settings_strong_enough;
/// Checks that a root `.npmrc` file exists.
mod root_exists;
/// Checks that the root `.npmrc` file is parseable.
mod root_parseable;
/// Orchestrates the npmrc config rule fan-out.
mod run;
/// Shared accessor helpers for the rule implementations.
mod support;

#[cfg(feature = "checks")]
pub use run::check;
