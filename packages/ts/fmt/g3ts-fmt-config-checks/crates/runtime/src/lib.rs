/// Shared helpers used by the fmt config checks.
mod common;
/// `format-check-script` rule for fmt config checks.
mod format_check_script;
/// `policy-configured` rule for fmt config checks.
mod policy_configured;
/// `prettier-config-present` rule for fmt config checks.
mod prettier_config_present;
/// `prettier-package-present` rule for fmt config checks.
mod prettier_package_present;
/// Top-level orchestration for fmt config checks.
mod run;
/// `syncpack-prettier-pin` rule for fmt config checks.
mod syncpack_prettier_pin;
/// `validate-runs-format-check` rule for fmt config checks.
mod validate_runs_format_check;

#[cfg(feature = "api")]
pub use run::check;
