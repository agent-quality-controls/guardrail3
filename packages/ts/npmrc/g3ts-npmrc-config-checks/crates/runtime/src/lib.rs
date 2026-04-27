mod duplicate_keys;
mod extra_settings_inventory;
mod required_settings_present;
mod required_settings_strong_enough;
mod root_exists;
mod root_parseable;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
