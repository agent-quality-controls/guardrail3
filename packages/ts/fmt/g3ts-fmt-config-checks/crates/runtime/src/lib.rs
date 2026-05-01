mod common;
mod format_check_fail_closed;
mod format_scripts;
mod policy_configured;
mod prettier_config_present;
mod prettier_package_present;
mod run;
mod syncpack_prettier_pin;
mod validate_runs_format_check;

#[cfg(feature = "api")]
pub use run::check;
