mod run;
mod support;
mod ts_npmrc_config_01_root_exists;
mod ts_npmrc_config_02_root_parseable;
mod ts_npmrc_config_03_duplicate_keys;
mod ts_npmrc_config_04_required_settings_present;
mod ts_npmrc_config_05_required_settings_strong_enough;
mod ts_npmrc_config_06_extra_settings_inventory;

#[cfg(feature = "checks")]
pub use run::check;
