use g3ts_hooks_contract_types as _;

/// Per-rule routing checks for the TS pre-commit hook.
mod routing;
/// Top-level orchestration of TS hook source checks.
mod run;

#[cfg(feature = "api")]
pub use run::{check, check_effective};
