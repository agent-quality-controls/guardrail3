use g3ts_eslint_config_checks_runtime as _;

/// Internal module `common`.
mod common;

#[cfg(feature = "checks")]
pub mod exists;
#[cfg(feature = "checks")]
pub mod no_console_error;
#[cfg(feature = "checks")]
pub mod no_explicit_any_error;
#[cfg(feature = "checks")]
pub mod parseable;
#[cfg(feature = "checks")]
pub mod project_service_enabled;
#[cfg(feature = "checks")]
pub mod run;
#[cfg(feature = "checks")]
pub mod ts_plugin_present;
