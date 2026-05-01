#[cfg(feature = "api")]
use g3ts_typecov_config_checks_runtime as _;

#[cfg(feature = "api")]
pub mod package_present;
#[cfg(feature = "api")]
pub mod policy_configured;
#[cfg(feature = "api")]
pub mod run;
#[cfg(feature = "api")]
pub mod script_present;
#[cfg(feature = "api")]
pub mod syncpack_type_coverage_pin;
#[cfg(feature = "api")]
pub mod threshold_fail_closed;
#[cfg(feature = "api")]
pub mod validate_runs_typecov;
