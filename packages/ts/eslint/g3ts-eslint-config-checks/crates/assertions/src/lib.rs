use g3ts_eslint_config_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod run;
#[cfg(feature = "checks")]
pub mod ts_eslint_config_01_exists;
#[cfg(feature = "checks")]
pub mod ts_eslint_config_02_parseable;
#[cfg(feature = "checks")]
pub mod ts_eslint_config_03_ts_plugin_present;
#[cfg(feature = "checks")]
pub mod ts_eslint_config_04_project_service_enabled;
#[cfg(feature = "checks")]
pub mod ts_eslint_config_05_no_explicit_any_error;
#[cfg(feature = "checks")]
pub mod ts_eslint_config_06_no_console_error;
