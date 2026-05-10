/// Shared helpers used by the typecov config checks.
mod common;
/// `package_present` rule for typecov config checks.
mod package_present;
/// `policy_configured` rule for typecov config checks.
mod policy_configured;
/// Top-level orchestration for typecov config checks.
mod run;
/// `script_present` rule for typecov config checks.
mod script_present;
/// `syncpack_type_coverage_pin` rule for typecov config checks.
mod syncpack_type_coverage_pin;
/// `threshold_fail_closed` rule for typecov config checks.
mod threshold_fail_closed;
/// `validate_runs_typecov` rule for typecov config checks.
mod validate_runs_typecov;

#[cfg(feature = "api")]
pub use run::check;
