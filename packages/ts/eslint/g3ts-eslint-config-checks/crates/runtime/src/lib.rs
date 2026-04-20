mod full_config;
mod run;
mod ts_eslint_config_01_exists;
mod ts_eslint_config_02_parseable;
mod ts_eslint_config_03_ts_plugin_present;
mod ts_eslint_config_04_project_service_enabled;
mod ts_eslint_config_05_no_explicit_any_error;
mod ts_eslint_config_06_no_console_error;

#[cfg(feature = "checks")]
pub use run::check;
