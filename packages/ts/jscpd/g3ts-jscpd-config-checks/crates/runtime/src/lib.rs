mod run;
mod support;
mod ts_jscpd_config_01_root_exists;
mod ts_jscpd_config_02_root_parseable;
mod ts_jscpd_config_03_threshold_zero;
mod ts_jscpd_config_04_absolute_true;
mod ts_jscpd_config_05_required_ignores;
mod ts_jscpd_config_06_format_and_inventory;

#[cfg(feature = "checks")]
pub use run::check;
