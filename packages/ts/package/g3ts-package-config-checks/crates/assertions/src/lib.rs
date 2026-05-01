use g3ts_package_config_checks_runtime as _;

#[cfg(feature = "checks")]
pub mod run;
#[cfg(feature = "checks")]
pub mod validate_script_fail_closed;
#[cfg(feature = "checks")]
pub mod validate_script_present;
