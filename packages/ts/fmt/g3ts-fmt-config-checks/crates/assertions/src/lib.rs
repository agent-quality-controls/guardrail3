#[cfg(feature = "api")]
use g3ts_fmt_config_checks_runtime as _;

#[cfg(feature = "api")]
pub mod format_check_fail_closed;
#[cfg(feature = "api")]
pub mod policy_configured;
#[cfg(feature = "api")]
pub mod prettier_package_present;
#[cfg(feature = "api")]
pub mod run;
