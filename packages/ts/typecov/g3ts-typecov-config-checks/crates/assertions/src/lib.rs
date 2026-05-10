#[cfg(feature = "api")]
use g3ts_typecov_config_checks_runtime as _;

/// Per-rule assertions for `package-present`.
#[cfg(feature = "api")]
pub mod package_present;
/// Per-rule assertions for `policy-configured`.
#[cfg(feature = "api")]
pub mod policy_configured;
/// Top-level runtime assertion helpers.
#[cfg(feature = "api")]
pub mod run;
/// Per-rule assertions for `script-present`.
#[cfg(feature = "api")]
pub mod script_present;
/// Per-rule assertions for `syncpack-type-coverage-pin`.
#[cfg(feature = "api")]
pub mod syncpack_type_coverage_pin;
/// Per-rule assertions for `threshold-fail-closed`.
#[cfg(feature = "api")]
pub mod threshold_fail_closed;
/// Per-rule assertions for `validate-runs-typecov`.
#[cfg(feature = "api")]
pub mod validate_runs_typecov;
